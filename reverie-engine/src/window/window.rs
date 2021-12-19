use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
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

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let text = encode("あいうえお");
    let text_len = text.len() as i32 - 1;
    let mut hdc = 0 as HDC;
    let mut pt = PAINTSTRUCT {
        hdc: 0 as HDC,
        fErase: FALSE as BOOL,
        rcPaint: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        fRestore: FALSE as BOOL,
        fIncUpdate: FALSE as BOOL,
        rgbReserved: [0; 32],
    };

    if msg == WM_DESTROY {
        PostQuitMessage(0);
    } else if msg == WM_PAINT {
        hdc = BeginPaint(hwnd, &mut pt);
        TextOutW(hdc, 10, 10, text.as_ptr(), text_len);
        EndPaint(hwnd, &mut pt);
    }
    return DefWindowProcW(hwnd, msg, wparam, lparam);
}

pub struct Window {
    pub window_handle: HWND,
}

impl Window {
    pub fn new(config: &WindowConfig) -> Self {
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

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.window_handle, SW_SHOW);
            UpdateWindow(self.window_handle);
        }
    }

    pub fn main_loop(&self) {
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

            if msg.message == WM_QUIT {
                break 'main_loop;
            }

            unsafe {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }
    }
}

pub struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
}

pub struct WindowConfigBuilder {
    title: Option<String>,
    width: u32,
    height: u32,
}

impl WindowConfigBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            width: 400,
            height: 300,
        }
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn build(self) -> WindowConfig {
        WindowConfig {
            title: self.title.unwrap_or("Reverie Engine".to_string()),
            width: self.width,
            height: self.height,
        }
    }
}
