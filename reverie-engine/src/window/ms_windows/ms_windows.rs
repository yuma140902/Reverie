use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::errhandlingapi::*;
use winapi::um::wingdi::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;

pub fn print_message(msg: &str) -> Result<i32, std::io::Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe { MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK) };
    if ret == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}

fn encode(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}

static mut OPENGL_RENDERING_CONTEXT_HANDLE_MUT: HGLRC = std::ptr::null_mut();

/// OpenGL用のHDCを取得する。使った後に[`release_opengl_hdc()`]を同じスレッドから呼び出す必要がある。
fn get_opengl_hdc(window_handle: HWND) -> Result<HDC, String> {
    let pfd = PIXELFORMATDESCRIPTOR {
        nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD,
        nVersion: 1,
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 24,
        cRedBits: 0,
        cRedShift: 0,
        cGreenBits: 0,
        cGreenShift: 0,
        cBlueBits: 0,
        cBlueShift: 0,
        cAlphaBits: 0,
        cAlphaShift: 0,
        cAccumBits: 0,
        cAccumRedBits: 0,
        cAccumGreenBits: 0,
        cAccumBlueBits: 0,
        cAccumAlphaBits: 0,
        cDepthBits: 32,
        cStencilBits: 0,
        cAuxBuffers: 0,
        iLayerType: PFD_MAIN_PLANE,
        bReserved: 0,
        dwLayerMask: 0,
        dwVisibleMask: 0,
        dwDamageMask: 0,
    };
    unsafe {
        let hdc: HDC = GetDC(window_handle);
        if hdc.is_null() {
            return Err("Fail GetDC()".to_string());
        }
        let pf: i32 = ChoosePixelFormat(hdc, &pfd);
        if pf == 0 {
            return Err(format!(
                "Fail ChoosePixelFormat(). GetLastError() = {}",
                GetLastError()
            ));
        }
        if SetPixelFormat(hdc, pf, &pfd) == FALSE {
            return Err(format!(
                "Fail SetPixelFormat(). GetLastError() = {}",
                GetLastError()
            ));
        }
        if OPENGL_RENDERING_CONTEXT_HANDLE_MUT.is_null() {
            OPENGL_RENDERING_CONTEXT_HANDLE_MUT = wglCreateContext(hdc);
            if OPENGL_RENDERING_CONTEXT_HANDLE_MUT.is_null() {
                return Err(format!(
                    "Fail wglCreateContext(). GetLastError() = {}",
                    GetLastError()
                ));
            }
        }
        if wglMakeCurrent(hdc, OPENGL_RENDERING_CONTEXT_HANDLE_MUT) == FALSE {
            return Err(format!(
                "Fail wglMakeCurrent(). GetLastError() = {}",
                GetLastError()
            ));
        }

        Ok(hdc)
    }
}

/// [`get_opengl_hdc()`]で取得したOpenGL用のHDCを解放する
fn release_opengl_hdc(window_handle: HWND, hdc: HDC) -> Result<(), String> {
    unsafe {
        if wglMakeCurrent(hdc, std::ptr::null_mut()) == FALSE {
            return Err(format!(
                "Fail wglMakeCurrent(). GetLastError() = {}",
                GetLastError()
            ));
        }
        if ReleaseDC(window_handle, hdc) == 0 {
            return Err("Fail ReleaseDC()".to_string());
        }
    }
    Ok(())
}

/// OpenGLの描画コンテキスト[`OPENGL_RENDERING_CONTEXT_HANDLE_MUT`]を削除する
fn exit_opengl() -> Result<(), String> {
    unsafe {
        if !OPENGL_RENDERING_CONTEXT_HANDLE_MUT.is_null() {
            if wglDeleteContext(OPENGL_RENDERING_CONTEXT_HANDLE_MUT) == FALSE {
                return Err(format!(
                    "Fail wglDeleteContext(). GetLastError() = {}",
                    GetLastError()
                ));
            }
        }
    }
    Ok(())
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let hdc: HDC;

    match msg {
        WM_PAINT => {
            // 無効リージョンをクリア(BeginPaint()を使用しないため)
            if ValidateRect(hwnd, std::ptr::null_mut()) == 0 {
                panic!("Fail ValidateRect()");
            }
            // OpenGL描画用のHDCを取得
            hdc = get_opengl_hdc(hwnd).unwrap();
            // ポリゴン描画
            // glRotatef(ang1, 1.0, 1.0, 1.0);
            // glClear(GL_COLOR_BUFFER_BIT);
            // ダブルバッファ切替
            SwapBuffers(hdc);
            // OpenGL描画用のHDCを解放
            release_opengl_hdc(hwnd, hdc).unwrap();
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        WM_QUIT => {
            exit_opengl().unwrap();
        }
        _ => {}
    }
    return DefWindowProcW(hwnd, msg, wparam, lparam);
}
pub struct MsWindowsWindow {
    pub window_handle: HWND,
}

impl crate::window::Window for MsWindowsWindow {
    fn new(config: &crate::window::WindowConfig) -> Self {
        let class_name = encode(&config.title);
        unsafe {
            let wnd = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: 0 as HINSTANCE,
                hIcon: LoadIconW(0 as HINSTANCE, IDI_APPLICATION),
                hCursor: LoadCursorW(0 as HINSTANCE, IDI_APPLICATION),
                hbrBackground: 16 as HBRUSH,
                lpszMenuName: 0 as LPCWSTR,
                lpszClassName: class_name.as_ptr(),
            };

            RegisterClassW(&wnd);
        }
        let window_handle = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                class_name.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                0,
                0,
                config.width as i32,
                config.height as i32,
                0 as HWND,
                0 as HMENU,
                0 as HINSTANCE,
                std::ptr::null_mut(),
            )
        };

        Self { window_handle }
    }

    fn show(&mut self) {
        unsafe {
            ShowWindow(self.window_handle, SW_SHOW);
            UpdateWindow(self.window_handle);
        }
    }

    fn main_loop(&mut self) {
        let mut msg = MSG {
            hwnd: 0 as HWND,
            message: 0 as UINT,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as DWORD,
            pt: POINT { x: 0, y: 0 },
        };

        'main_loop: loop {
            let ret = unsafe { GetMessageW(&mut msg, 0 as HWND, 0, 0) };
            if ret == 0 {
                break 'main_loop;
            }

            unsafe {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }
    }
}
