use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::{null_mut};
use std::process::exit;

// Struktur für den Zustand des Programms
struct AppState {
    phone: *mut c_void,
    last_call_index: i32,
    last_call_id: Option<String>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            phone: null_mut(),
            last_call_index: -1,
            last_call_id: None,
        }
    }
}

extern "C" {
    fn phone_create_with_system_nameserver(
        name: *const c_char,
        stunserver: *const *const c_char,
        count: usize,
    ) -> *mut c_void;
    fn phone_destroy(phone: *mut c_void);
    fn phone_set_log_function(phone: *mut c_void, callback: extern "C" fn(*const c_char));
    fn phone_register_on_registration_state_callback(
        phone: *mut c_void,
        callback: extern "C" fn(c_int, c_int, *mut c_void),
        ctx: *mut c_void,
    );
    fn phone_register_on_incoming_call_index_callback(
        phone: *mut c_void,
        callback: extern "C" fn(c_int, *mut c_void),
        ctx: *mut c_void,
    );
    fn phone_register_on_incoming_call_id_callback(
        phone: *mut c_void,
        callback: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );
    fn phone_register_on_call_state_index_callback(
        phone: *mut c_void,
        callback: extern "C" fn(c_int, c_int, *mut c_void),
        ctx: *mut c_void,
    );
    fn phone_register_on_call_state_id_callback(
        phone: *mut c_void,
        callback: extern "C" fn(*const c_char, c_int, *mut c_void),
        ctx: *mut c_void,
    );
    fn phone_configure_opus(phone: *mut c_void, enabled: c_int, bitrate: c_int, frequency: c_int) -> c_int;
    fn phone_make_call(phone: *mut c_void, number: *const c_char) -> c_int;
    fn phone_answer_call_index(phone: *mut c_void, call_index: c_int) -> c_int;
    fn phone_answer_call_id(phone: *mut c_void, call_id: *const c_char) -> c_int;
    fn phone_hangup_call_index(phone: *mut c_void, call_index: c_int) -> c_int;
    fn phone_hangup_call_id(phone: *mut c_void, call_id: *const c_char) -> c_int;
    fn phone_refresh_audio_devices();
    fn phone_last_error() -> *const c_char;
    fn phone_disconnect(phone: *mut c_void) -> c_int;
}

// Fehlernachricht abrufen
fn die(message: &str) -> ! {
    eprintln!("Error: {}", message);
    exit(1);
}

fn get_last_error_message() -> String {
    unsafe {
        let error_ptr = phone_last_error();
        if error_ptr.is_null() {
            return "Unknown error".to_string();
        }
        CStr::from_ptr(error_ptr).to_string_lossy().into()
    }
}

// Log-Rückruf
extern "C" fn log_function(message: *const c_char) {
    if !message.is_null() {
        let message = unsafe { CStr::from_ptr(message).to_string_lossy() };
        println!("[LOG]: {}", message);
    } else {
        println!("[LOG]: <empty>");
    }
}

// Rückrufe für Programmevents
extern "C" fn on_registration_state(is_registered: c_int, state: c_int, _ctx: *mut c_void) {
    println!(
        "Registration state: is_registered = {}, state = {}",
        is_registered, state
    );
}

extern "C" fn on_incoming_call_index(call_index: c_int, ctx: *mut c_void) {
    if !ctx.is_null() {
        println!(
            "Incoming call at index: {} (context pointer: {:p})",
            call_index, ctx
        );
    } else {
        println!("Incoming call at index: {}", call_index);
    }
}

extern "C" fn on_incoming_call_id(call_id: *const c_char, ctx: *mut c_void) {
    let call_id = unsafe { CStr::from_ptr(call_id).to_string_lossy().into_owned() };
    if !ctx.is_null() {
        println!(
            "Incoming call with ID: {} (context pointer: {:p})",
            call_id, ctx
        );
    } else {
        println!("Incoming call with ID: {}", call_id);
    }
}

// Anrufstatus-Rückrufe
extern "C" fn on_call_state_index(call_index: c_int, state: c_int, ctx: *mut c_void) {
    println!(
        "Call state - index: {}, state: {} (context pointer: {:p})",
        call_index, state, ctx
    );
}

extern "C" fn on_call_state_id(call_id: *const c_char, state: c_int, ctx: *mut c_void) {
    let call_id = unsafe { CStr::from_ptr(call_id).to_string_lossy().into_owned() };
    println!(
        "Call state - ID: {}, state: {} (context pointer: {:p})",
        call_id, state, ctx
    );
}

// Benutzereingaben ausführen
fn execute_command(command: char, state: &mut AppState) {
    match command {
        'c' => {
            print!("Enter number to call: ");
            io::stdout().flush().unwrap();

            let mut number = String::new();
            io::stdin().read_line(&mut number).expect("Failed to read input.");
            let c_string = CString::new(number.trim()).unwrap();

            unsafe {
                if phone_make_call(state.phone, c_string.as_ptr()) != 0 {
                    println!("Failed to make call: {}", get_last_error_message());
                } else {
                    println!("Successfully initiated the call.");
                }
            }
        }
        'a' => {
            print!("Enter call index to answer: ");
            io::stdout().flush().unwrap();

            let mut index = String::new();
            io::stdin().read_line(&mut index).expect("Failed to read input.");
            let call_index: c_int = index.trim().parse().unwrap_or(-1);

            unsafe {
                if phone_answer_call_index(state.phone, call_index) != 0 {
                    println!("Failed to answer call: {}", get_last_error_message());
                } else {
                    println!("Successfully answered the call.");
                }
            }
        }
        'A' => {
            print!("Enter call ID to answer: ");
            io::stdout().flush().unwrap();

            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read input.");
            let c_string = CString::new(id.trim()).unwrap();

            unsafe {
                if phone_answer_call_id(state.phone, c_string.as_ptr()) != 0 {
                    println!("Failed to answer call: {}", get_last_error_message());
                } else {
                    println!("Successfully answered the call.");
                }
            }
        }
        'h' => {
            print!("Enter call index to hang up: ");
            io::stdout().flush().unwrap();

            let mut index = String::new();
            io::stdin().read_line(&mut index).expect("Failed to read input.");
            let call_index: c_int = index.trim().parse().unwrap_or(-1);

            unsafe {
                if phone_hangup_call_index(state.phone, call_index) != 0 {
                    println!("Failed to hang up call: {}", get_last_error_message());
                } else {
                    println!("Successfully hung up the call.");
                }
            }
        }
        'H' => {
            print!("Enter call ID to hang up: ");
            io::stdout().flush().unwrap();

            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read input.");
            let c_string = CString::new(id.trim()).unwrap();

            unsafe {
                if phone_hangup_call_id(state.phone, c_string.as_ptr()) != 0 {
                    println!("Failed to hang up call: {}", get_last_error_message());
                } else {
                    println!("Successfully hung up the call.");
                }
            }
        }
        'e' => {
            unsafe {
                phone_refresh_audio_devices();
                println!("Audio devices refreshed.");
            }
        }
        'q' => {
            println!("Exiting...");
        }
        _ => {
            println!("Unknown command.");
        }
    }
}

fn main() {
    let mut state = AppState::new();
    let stunserver = [CString::new("stun.t-online.de").unwrap()];
    let stunserver_ptrs: Vec<*const c_char> = stunserver.iter().map(|s| s.as_ptr()).collect();

    unsafe {
        state.phone = phone_create_with_system_nameserver(
            CString::new("Rust CLI").unwrap().as_ptr(),
            stunserver_ptrs.as_ptr(),
            stunserver.len(),
        );

        if state.phone.is_null() {
            die("Failed to create phone system.");
        }

        phone_set_log_function(state.phone, log_function);
        phone_register_on_registration_state_callback(state.phone, on_registration_state, null_mut());
        phone_register_on_incoming_call_index_callback(state.phone, on_incoming_call_index, null_mut());
        phone_register_on_incoming_call_id_callback(state.phone, on_incoming_call_id, null_mut());
        phone_register_on_call_state_index_callback(state.phone, on_call_state_index, null_mut());
        phone_register_on_call_state_id_callback(state.phone, on_call_state_id, null_mut());

        if phone_configure_opus(state.phone, 1, 8, 16000) != 0 {
            die("Failed to configure OPUS codec");
        }
    }

    // CLI-Kommandoschleife
    let mut command = String::new();
    while command.trim() != "q" {
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        command.clear();
        io::stdin().read_line(&mut command).expect("Failed to read command");
        if let Some(cmd) = command.trim().chars().next() {
            execute_command(cmd, &mut state);
        }
    }

    unsafe {
        phone_destroy(state.phone);
    }
}