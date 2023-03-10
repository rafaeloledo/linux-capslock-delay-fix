default  hidden partial modifier_keys
xkb_symbols "capslock" {
    replace key <CAPS> { [ Caps_Lock ] };
    modifier_map Lock { Caps_Lock };
};

hidden partial modifier_keys
xkb_symbols "shiftlock" {
    replace key <CAPS> { [ Shift_Lock ] };
    modifier_map Shift { Shift_Lock };
};

hidden partial modifier_keys
xkb_symbols "grouplock" {
    replace key <CAPS> { [ ISO_Next_Group, Caps_Lock ] };
};

hidden partial modifier_keys
xkb_symbols "groupshift" {
    key <CAPS> {
        type[Group1] = "PC_ALT_LEVEL2",
        [ Mode_switch, Caps_Lock ]
    };
};

hidden partial modifier_keys
xkb_symbols "swapescape" {
    key <CAPS> { [ Escape ] };
    key <ESC>  { [ Caps_Lock ] };
};

hidden partial modifier_keys
xkb_symbols "escape" {
    key <CAPS> { [ Escape ] };
};

hidden partial modifier_keys
xkb_symbols "escape_shifted_capslock" {
    key <CAPS> {
        type[Group1] = "TWO_LEVEL",
        symbols[Group1] = [ Escape, Caps_Lock ],
        actions[Group1] = [ NoAction(), LockMods(modifiers = Lock) ]
    };
};

hidden partial modifier_keys
xkb_symbols "backspace" {
    key <CAPS> { [ BackSpace ] };
};

hidden partial modifier_keys
xkb_symbols "super" {
    key <CAPS> { [ Super_L ] };
    modifier_map Mod4 { <CAPS> };
};

hidden partial modifier_keys
xkb_symbols "hyper" {
    key <CAPS> { [ Hyper_L ] };
    modifier_map Mod4 { <CAPS> };
};

hidden partial modifier_keys
xkb_symbols "menu" {
    key <CAPS> { [ Menu ] };
};

hidden partial modifier_keys
xkb_symbols "numlock" {
    key <CAPS> { [ Num_Lock ] };
};

// This changes the <CAPS> key to become a Control modifier,
// but it will still produce the Caps_Lock keysym.
hidden partial modifier_keys
xkb_symbols "ctrl_modifier" {
    key <CAPS> {
        type="ALPHABETIC",
        repeat=No,
        symbols[Group1]= [ Caps_Lock, Caps_Lock ],
        actions[Group1]= [ LockMods(modifiers=Lock), LockMods(modifiers=Shift+Lock,affect=unlock) ]
    };
};

hidden partial modifier_keys
xkb_symbols "none" {
    key <CAPS> { [ VoidSymbol ] };
};
