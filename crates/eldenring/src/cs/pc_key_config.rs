#[repr(C)]
#[shared::singleton("CSPcKeyConfig")]
/// Source of name: RTTI
pub struct CSPcKeyConfig {
    vftable: usize,
    unk8: [u8; 0x438],
    key_mappings: [KeyAssign; 54],
}

impl CSPcKeyConfig {
    /// Returns the keybindings for the given action
    pub fn key_assign(&self, id: KeyAssignID) -> &KeyAssign {
        &self.key_mappings[id as usize]
    }

    /// Returns the keybindings for the given action
    pub fn key_assign_mut(&mut self, id: KeyAssignID) -> &mut KeyAssign {
        &mut self.key_mappings[id as usize]
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// ID in KeyAssignParam, and index in [CSPcKeyConfig::key_mappings]. Used to look up the
/// keybindings for an action
pub enum KeyAssignID {
    MovementControl = 0,
    MoveForwards = 1,
    MoveBackwards = 2,
    MoveLeft = 3,
    MoveRight = 4,
    CrouchStandUp = 5,
    BackstepDodgeRollDash = 6,
    Jump = 7,
    MoveCameraChangeTargetUp = 8,
    MoveCameraChangeTargetDown = 9,
    MoveCameraChangeTargetLeft = 10,
    MoveCameraChangeTargetRight = 11,
    ResetCameraLockOnRemoveTarget = 12,
    SwitchSorceryIncantation = 13,
    SwitchItem = 14,
    SwitchRightHandArmament = 15,
    SwitchLeftHandArmament = 16,
    AttackRHTwoHandedArmament = 17,
    StrongAttackRH2HArmament = 18,
    GuardLHArmament = 19,
    Skill = 20,
    UseItem = 21,
    EventActionExamineOpenEtc = 22,
    MainMenu = 24,
    Map = 25,
}

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// Current gamepad/keyboard/mouse bindings for a single action
pub struct KeyAssign {
    pub pad_key_id: CSPadKey,
    pub keyboard_key_id: CSKeyboardKey,
    pub keyboard_modify_key: Option<CSModifierKey>,
    pub mouse_key_id: Option<CSMouseKey>,
    pub mouse_modify_key: Option<CSModifierKey>,
}

#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// Haven't looked into this sorry
/// Source of name: CS_PAD_KEY enum in paramdefs
pub struct CSPadKey(pub i32);

#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// Mostly a subset DirectInput keys but plus 69 for some reason
/// Source of name: CS_KEYBOARD_KEY enum in paramdefs
pub enum CSKeyboardKey {
    None = -1,
    Escape = 70,
    Number1 = 71,
    Number2 = 72,
    Number3 = 73,
    Number4 = 74,
    Number5 = 75,
    Number6 = 76,
    Number7 = 77,
    Number8 = 78,
    Number9 = 79,
    Number0 = 80,
    Minus = 81,
    Equals = 82,
    Backspace = 83,
    Tab = 84,
    Q = 85,
    W = 86,
    E = 87,
    R = 88,
    T = 89,
    Y = 90,
    U = 91,
    I = 92,
    O = 93,
    P = 94,
    LeftBracket = 95,
    RightBracket = 96,
    Return = 97,
    LeftControl = 98,
    A = 99,
    S = 100,
    D = 101,
    F = 102,
    G = 103,
    H = 104,
    J = 105,
    K = 106,
    L = 107,
    Semicolon = 108,
    Apostrophe = 109,
    Grave = 110,
    LeftShift = 111,
    Backslash = 112,
    Z = 113,
    X = 114,
    C = 115,
    V = 116,
    B = 117,
    N = 118,
    M = 119,
    Comma = 120,
    Period = 121,
    Slash = 122,
    RightShift = 123,
    NumpadStar = 124,
    LeftAlt = 125,
    Space = 126,
    Capslock = 127,
    UpArrow = 190,
    LeftArrow = 195,
    RightArrow = 193,
    DownArrow = 192,
}

#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// Source of name: CS_MOUSE_KEY enum in paramdefs
pub enum CSMouseKey {
    Right = 1,
    Left = 2,
    Middle = 8,
    ScrollUp = 9,
    ScrollDown = 10,
}

#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// Source of name: CS_MODIFIER_KEY enum in paramdefs
pub enum CSModifierKey {
    Ctrl = 1,
    Alt = 2,
    Shift = 3,
}
