// automatically generated by rust-bindgen


#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum SwkbdType {
    SWKBD_TYPE_NORMAL = 0,
    SWKBD_TYPE_QWERTY = 1,
    SWKBD_TYPE_NUMPAD = 2,
    SWKBD_TYPE_WESTERN = 3,
}
pub const SWKBD_NOTBLANK_NOTEMPTY: SwkbdValidInput = SwkbdValidInput::SWKBD_NOTEMPTY_NOTBLANK;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum SwkbdValidInput {
    SWKBD_ANYTHING = 0,
    SWKBD_NOTEMPTY = 1,
    SWKBD_NOTEMPTY_NOTBLANK = 2,
    SWKBD_NOTBLANK = 3,
    SWKBD_FIXEDLEN = 4,
}
pub const SWKBD_BUTTON_CONFIRM: SwkbdButton = SwkbdButton::SWKBD_BUTTON_RIGHT;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum SwkbdButton {
    SWKBD_BUTTON_LEFT = 0,
    SWKBD_BUTTON_MIDDLE = 1,
    SWKBD_BUTTON_RIGHT = 2,
    SWKBD_BUTTON_NONE = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum SwkbdPasswordMode {
    SWKBD_PASSWORD_NONE = 0,
    SWKBD_PASSWORD_HIDE = 1,
    SWKBD_PASSWORD_HIDE_DELAY = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed1 {
    SWKBD_FILTER_DIGITS = 1,
    SWKBD_FILTER_AT = 2,
    SWKBD_FILTER_PERCENT = 4,
    SWKBD_FILTER_BACKSLASH = 8,
    SWKBD_FILTER_PROFANITY = 16,
    SWKBD_FILTER_CALLBACK = 32,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed2 {
    SWKBD_PARENTAL = 1,
    SWKBD_DARKEN_TOP_SCREEN = 2,
    SWKBD_PREDICTIVE_INPUT = 4,
    SWKBD_MULTILINE = 8,
    SWKBD_FIXED_WIDTH = 16,
    SWKBD_ALLOW_HOME = 32,
    SWKBD_ALLOW_RESET = 64,
    SWKBD_ALLOW_POWER = 128,
    SWKBD_DEFAULT_QWERTY = 512,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum SwkbdCallbackResult {
    SWKBD_CALLBACK_OK = 0,
    SWKBD_CALLBACK_CLOSE = 1,
    SWKBD_CALLBACK_CONTINUE = 2,
}
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum SwkbdResult {
    SWKBD_NONE = -1,
    SWKBD_INVALID_INPUT = -2,
    SWKBD_OUTOFMEM = -3,
    SWKBD_D0_CLICK = 0,
    SWKBD_D1_CLICK0 = 1,
    SWKBD_D1_CLICK1 = 2,
    SWKBD_D2_CLICK0 = 3,
    SWKBD_D2_CLICK1 = 4,
    SWKBD_D2_CLICK2 = 5,
    SWKBD_HOMEPRESSED = 10,
    SWKBD_RESETPRESSED = 11,
    SWKBD_POWERPRESSED = 12,
    SWKBD_PARENTAL_OK = 20,
    SWKBD_PARENTAL_FAIL = 21,
    SWKBD_BANNED_INPUT = 30,
}
#[repr(C)]
#[derive(Copy)]
pub struct SwkbdDictWord {
    pub reading: [u16; 41usize],
    pub word: [u16; 41usize],
    pub language: u8,
    pub all_languages: u8,
}
impl ::core::clone::Clone for SwkbdDictWord {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SwkbdDictWord {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type SwkbdCallbackFn =
    ::core::option::Option<unsafe extern "C" fn(user: *mut ::libc::c_void,
                                                  ppMessage: *mut *const u8,
                                                  text: *const u8,
                                                  textlen: usize)
                                                  -> SwkbdCallbackResult>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct SwkbdStatusData {
    pub data: [u32; 17usize],
}
impl ::core::default::Default for SwkbdStatusData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct SwkbdLearningData {
    pub data: [u32; 10523usize],
}
impl ::core::clone::Clone for SwkbdLearningData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SwkbdLearningData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct SwkbdExtra {
    pub initial_text: *const u8,
    pub dict: *const SwkbdDictWord,
    pub status_data: *mut SwkbdStatusData,
    pub learning_data: *mut SwkbdLearningData,
    pub callback: SwkbdCallbackFn,
    pub callback_user: *mut ::libc::c_void,
}
impl ::core::default::Default for SwkbdExtra {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct SwkbdState {
    pub type_: i32,
    pub num_buttons_m1: i32,
    pub valid_input: i32,
    pub password_mode: i32,
    pub is_parental_screen: i32,
    pub darken_top_screen: i32,
    pub filter_flags: u32,
    pub save_state_flags: u32,
    pub max_text_len: u16,
    pub dict_word_count: u16,
    pub max_digits: u16,
    pub button_text: [[u16; 17usize]; 3usize],
    pub numpad_keys: [u16; 2usize],
    pub hint_text: [u16; 65usize],
    pub predictive_input: u8,
    pub multiline: u8,
    pub fixed_width: u8,
    pub allow_home: u8,
    pub allow_reset: u8,
    pub allow_power: u8,
    pub unknown: u8,
    pub default_qwerty: u8,
    pub button_submits_text: [u8; 4usize],
    pub language: u16,
    pub initial_text_offset: i32,
    pub dict_offset: i32,
    pub initial_status_offset: i32,
    pub initial_learning_offset: i32,
    pub shared_memory_size: usize,
    pub version: u32,
    pub result: SwkbdResult,
    pub status_offset: i32,
    pub learning_offset: i32,
    pub text_offset: i32,
    pub text_length: u16,
    pub callback_result: i32,
    pub callback_msg: [u16; 257usize],
    pub skip_at_check: u8,
    pub union: _bindgen_data_1_,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _bindgen_data_1_ {
    pub reserved: [u8; 171usize],
    pub extra: SwkbdExtra,
}
impl ::core::clone::Clone for SwkbdState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SwkbdState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
extern "C" {
    pub fn swkbdInit(swkbd: *mut SwkbdState,
                     type_: SwkbdType,
                     numButtons: i32,
                     maxTextLength: i32);
    pub fn swkbdSetFeatures(swkbd: *mut SwkbdState, features: u32);
    pub fn swkbdSetHintText(swkbd: *mut SwkbdState, text: *const u8);
    pub fn swkbdSetButton(swkbd: *mut SwkbdState,
                          button: SwkbdButton,
                          text: *const u8,
                          submit: u8);
    pub fn swkbdSetInitialText(swkbd: *mut SwkbdState, text: *const u8);
    pub fn swkbdSetDictWord(word: *mut SwkbdDictWord,
                            reading: *const u8,
                            text: *const u8);
    pub fn swkbdSetDictionary(swkbd: *mut SwkbdState,
                              dict: *const SwkbdDictWord,
                              wordCount: i32);
    pub fn swkbdSetStatusData(swkbd: *mut SwkbdState,
                              data: *mut SwkbdStatusData,
                              in_: u8,
                              out: u8);
    pub fn swkbdSetLearningData(swkbd: *mut SwkbdState,
                                data: *mut SwkbdLearningData,
                                in_: u8,
                                out: u8);
    pub fn swkbdSetFilterCallback(swkbd: *mut SwkbdState,
                                  callback: SwkbdCallbackFn,
                                  user: *mut ::libc::c_void);
    pub fn swkbdInputText(swkbd: *mut SwkbdState,
                          buf: *mut u8,
                          bufsize: usize)
                          -> SwkbdButton;
}
