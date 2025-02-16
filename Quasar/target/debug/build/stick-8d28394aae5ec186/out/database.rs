pub(super) fn ctlr_desc(
    bus: u16, vendor: u16, product: u16, ver: u16
) -> &'static CtlrDescriptor
{
    match (bus, vendor, product, ver) {
        (0x0300, 0x00F0, 0x0300, 0x0001) => &CtlrDescriptor {
            name: "RetroPad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::Prev, 2),
                (&Event::Next, 3),
                (&Event::ActionV, 4),
                (&Event::ActionB, 5),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x00F0, 0xF100, 0x0001) => &CtlrDescriptor {
            name: "Super RetroPort",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::Prev, 2),
                (&Event::Next, 3),
                (&Event::ActionV, 4),
                (&Event::ActionB, 5),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0220, 0x0090, 0x1101) => &CtlrDescriptor {
            name: "8BitDo NES30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0300, 0x0300, 0x0200) => &CtlrDescriptor {
            name: "Miroof",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0B04, 0x3365, 0x0001) => &CtlrDescriptor {
            name: "Competition Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Prev, 2),
                (&Event::Next, 3),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D05, 0x0308, 0x1001) => &CtlrDescriptor {
            name: "Nostromo n45 Dual Analog Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Home, 9),
                (&Event::Next, 10),
                (&Event::CamPush, 11),
                (&Event::JoyPush, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x0900, 0x1001) => &CtlrDescriptor {
            name: "Natec Genesis P44",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x0D00, 0x0001) => &CtlrDescriptor {
            name: "hori",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionV, 2),
                (&Event::BumperL, 3),
                (&Event::ActionB, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
            ],
            trigbtns: &[
                (&Event::JoyX, 4),
                (&Event::JoyY, 5),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x1000, 0x1101) => &CtlrDescriptor {
            name: "HORI CO. LTD. FIGHTING STICK 3",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x1600, 0x0001) => &CtlrDescriptor {
            name: "Hori Real Arcade Pro.EX-SE (Xbox 360)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x2200, 0x1101) => &CtlrDescriptor {
            name: "HORI CO. LTD. REAL ARCADE Pro.V3",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x5E00, 0x1101) => &CtlrDescriptor {
            name: "Hori Fighting Commander 4 (PS4)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x5F00, 0x1101) => &CtlrDescriptor {
            name: "Hori Fighting Commander 4 (PS3)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x6600, 0x1101) => &CtlrDescriptor {
            name: "HORIPAD 4 (PS4)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x6700, 0x0101) => &CtlrDescriptor {
            name: "HORIPAD ONE",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x6A00, 0x1101) => &CtlrDescriptor {
            name: "HORI CO. LTD. Real Arcade Pro.4",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x6B00, 0x1101) => &CtlrDescriptor {
            name: "HORI CO. LTD. Real Arcade Pro.4",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x6E00, 0x1101) => &CtlrDescriptor {
            name: "HORIPAD 4 (PS3)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x8400, 0x1101) => &CtlrDescriptor {
            name: "HORI Fighting Commander",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x8500, 0x1001) => &CtlrDescriptor {
            name: "HORI Fighting Commander",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x8600, 0x0201) => &CtlrDescriptor {
            name: "Hori Fighting Commander",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0x9200, 0x1101) => &CtlrDescriptor {
            name: "Hori Pokken Tournament DX Pro Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0xAA00, 0x1101) => &CtlrDescriptor {
            name: "HORI Real Arcade Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0xC100, 0x1101) => &CtlrDescriptor {
            name: "HORI CO. LTD. HORIPAD S",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 13),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x0D0F, 0xEE00, 0x1101) => &CtlrDescriptor {
            name: "HORIPAD mini4",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x1000, 0x8200, 0x1101) => &CtlrDescriptor {
            name: "Akishop Customs PS360+ v1.66",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Home, 8),
                (&Event::Next, 9),
                (&Event::Prev, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x1008, 0x0100, 0x1001) => &CtlrDescriptor {
            name: "Twin USB PS2 Adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x1008, 0x01E5, 0x1001) => &CtlrDescriptor {
            name: "NEXT SNES Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x1008, 0x0300, 0x1001) => &CtlrDescriptor {
            name: "USB Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x120C, 0x0500, 0x0001) => &CtlrDescriptor {
            name: "Manta Dualshock 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x120C, 0x0500, 0x1001) => &CtlrDescriptor {
            name: "AxisPad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::ActionA, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x120C, 0x100E, 0x1101) => &CtlrDescriptor {
            name: "ZEROPLUS P4 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x120C, 0x101E, 0x1101) => &CtlrDescriptor {
            name: "ZEROPLUS P4 Wired Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x242E, 0x8816, 0x0101) => &CtlrDescriptor {
            name: "Hyperkin X91",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x242F, 0x2D00, 0x1101) => &CtlrDescriptor {
            name: "JYS Wireless Adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x242F, 0x7300, 0x1101) => &CtlrDescriptor {
            name: "Mayflash Magic NS",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 3),
                (&Event::ActionB, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::Home, 12),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x242F, 0x8A00, 0x1101) => &CtlrDescriptor {
            name: "JYS Wireless Adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 3),
                (&Event::ActionB, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::Home, 12),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x2509, 0x0500, 0x0001) => &CtlrDescriptor {
            name: "Sony PS2 pad with SmartJoy adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 8),
                (&Event::Prev, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x2509, 0x6688, 0x0001) => &CtlrDescriptor {
            name: "MP-8866 Super Dual Box",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 8),
                (&Event::Prev, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x2509, 0xE803, 0x0101) => &CtlrDescriptor {
            name: "Mayflash Wii Classic Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
                (&|p| Event::TriggerL(p * 0.5 + 0.5), 4, None),
                (&|p| Event::TriggerR(p * 0.5 + 0.5), 5, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x2609, 0x8888, 0x0001) => &CtlrDescriptor {
            name: "Cyber Gadget GameCube Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperR, 6),
                (&Event::Next, 7),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 4, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x2804, 0x0140, 0x0001) => &CtlrDescriptor {
            name: "Gravis Gamepad Pro USB ",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x300F, 0x0B01, 0x1001) => &CtlrDescriptor {
            name: "Jess Tech GGE909 PC Recoil Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x300F, 0x1001, 0x1001) => &CtlrDescriptor {
            name: "Jess Tech Dual Analog Rumble Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::ActionA, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x300F, 0x1201, 0x1001) => &CtlrDescriptor {
            name: "Saitek P380",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::ActionA, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x300F, 0x1211, 0x1101) => &CtlrDescriptor {
            name: "QanBa Arcade JoyStick",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionH, 1),
                (&Event::ActionA, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 5),
                (&Event::BumperR, 7),
                (&Event::Home, 8),
                (&Event::Next, 9),
                (&Event::Prev, 10),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 6),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3215, 0x0009, 0x1101) => &CtlrDescriptor {
            name: "Razer Serval",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3215, 0x0010, 0x1101) => &CtlrDescriptor {
            name: "Razer RAIJU",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3215, 0x0011, 0x1101) => &CtlrDescriptor {
            name: "Razer Raion Fightpad for PS4",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3215, 0x0104, 0x1101) => &CtlrDescriptor {
            name: "Razer Panthera (PS4)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3215, 0x0204, 0x1101) => &CtlrDescriptor {
            name: "Razer Panthera (PS3)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3215, 0x030A, 0x0101) => &CtlrDescriptor {
            name: "Razer Wildcat",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3215, 0x0507, 0x0001) => &CtlrDescriptor {
            name: "Razer Raiju Mobile",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Home, 21),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x341A, 0x05F7, 0x1001) => &CtlrDescriptor {
            name: "GameCube {HuiJia USB box}",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperR, 7),
                (&Event::Next, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x341A, 0x0908, 0x1001) => &CtlrDescriptor {
            name: "SL-6566",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x341A, 0x3608, 0x1101) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3512, 0x12AB, 0x1001) => &CtlrDescriptor {
            name: "8BitDo NES30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3512, 0x20AB, 0x1001) => &CtlrDescriptor {
            name: "8BitDo SNES30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x1647, 0x1004) => &CtlrDescriptor {
            name: "Mad Catz Wired Xbox 360 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x1888, 0x1001) => &CtlrDescriptor {
            name: "MadCatz PC USB Wired Stick 8818",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x3847, 0x9004) => &CtlrDescriptor {
            name: "Mad Catz Wired Xbox 360 Controller (SFIV)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x3888, 0x1001) => &CtlrDescriptor {
            name: "MadCatz PC USB Wired Stick 8838",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x5032, 0x1101) => &CtlrDescriptor {
            name: "Mad Catz FightPad PRO (PS3)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x5082, 0x1101) => &CtlrDescriptor {
            name: "Mad Catz FightPad PRO (PS4)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x8034, 0x1101) => &CtlrDescriptor {
            name: "Mad Catz fightstick (PS3)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x8084, 0x1101) => &CtlrDescriptor {
            name: "Mad Catz fightstick (PS4)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x8433, 0x1101) => &CtlrDescriptor {
            name: "Mad Catz FightStick TE S+ (PS3)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3807, 0x8483, 0x1101) => &CtlrDescriptor {
            name: "Mad Catz FightStick TE S+ (PS4)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3810, 0x3014, 0x7501) => &CtlrDescriptor {
            name: "SteelSeries Stratus Duo",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3810, 0x3114, 0x7501) => &CtlrDescriptor {
            name: "SteelSeries Stratus Duo",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x3B07, 0x04A1, 0x0001) => &CtlrDescriptor {
            name: "Suncom SFX Plus for USB",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 6),
                (&Event::Prev, 7),
                (&Event::Next, 8),
                (&Event::BumperR, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x450C, 0x2043, 0x1001) => &CtlrDescriptor {
            name: "XEOX Gamepad SL-6556-BK",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4513, 0x0830, 0x1001) => &CtlrDescriptor {
            name: "NYKO CORE",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4575, 0x2211, 0x1001) => &CtlrDescriptor {
            name: "SZMY-POWER PC Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0x6802, 0x1001) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::Prev, 0),
                (&Event::JoyPush, 1),
                (&Event::CamPush, 2),
                (&Event::Next, 3),
                (&Event::DpadUp, 4),
                (&Event::DpadRight, 5),
                (&Event::DpadDown, 6),
                (&Event::DpadLeft, 7),
                (&Event::BumperL, 10),
                (&Event::BumperR, 11),
                (&Event::ActionV, 12),
                (&Event::ActionB, 13),
                (&Event::ActionA, 14),
                (&Event::ActionH, 15),
                (&Event::Home, 16),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0x6802, 0x1081) => &CtlrDescriptor {
            name: "Shanwan PlayStation3 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 16),
                (&Event::ActionA, 17),
                (&Event::ActionV, 19),
                (&Event::ActionH, 20),
                (&Event::BumperL, 22),
                (&Event::BumperR, 23),
                (&Event::Nil, 24),
                (&Event::Nil, 25),
                (&Event::Prev, 26),
                (&Event::Next, 27),
                (&Event::Home, 28),
                (&Event::JoyPush, 29),
                (&Event::CamPush, 30),
                (&Event::DpadUp, 256),
                (&Event::DpadDown, 257),
                (&Event::DpadLeft, 258),
                (&Event::DpadRight, 259),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0x6802, 0x1101) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::Prev, 0),
                (&Event::JoyPush, 1),
                (&Event::CamPush, 2),
                (&Event::Next, 3),
                (&Event::DpadUp, 4),
                (&Event::DpadRight, 5),
                (&Event::DpadDown, 6),
                (&Event::DpadLeft, 7),
                (&Event::BumperL, 10),
                (&Event::BumperR, 11),
                (&Event::ActionV, 12),
                (&Event::ActionB, 13),
                (&Event::ActionA, 14),
                (&Event::ActionH, 15),
                (&Event::Home, 16),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 12, None, None),
                (&Event::TriggerR, 13, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0x6802, 0x1181) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::DpadRight, 16),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xA00B, 0x1101) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xA00B, 0x1181) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xC405, 0x1101) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xC405, 0x1181) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xCC09, 0x0001) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xCC09, 0x1101) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xCC09, 0x1181) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4C05, 0xDA0C, 0x1101) => &CtlrDescriptor {
            name: "Playstation Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x00B3, 0x1001) => &CtlrDescriptor {
            name: "Thrustmaster Firestorm Dual Power",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Home, 8),
                (&Event::Prev, 9),
                (&Event::Next, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x0404, 0x1101) => &CtlrDescriptor {
            name: "Thrustmaster Warthog Throttle",
            deadzone: None,
            buttons: &[
                (&Event::MousePush, 0),
                (&Event::MicPush, 1),
                (&Event::MicUp, 2),
                (&Event::MicLeft, 3),
                (&Event::MicDown, 4),
                (&Event::MicRight, 5),
                (&Event::SpeedbrakeForward, 6),
                (&Event::SpeedbrakeBackward, 7),
                (&Event::BoatForward, 8),
                (&Event::BoatBackward, 9),
                (&Event::ChinaForward, 10),
                (&Event::ChinaBackward, 11),
                (&Event::PinkyForward, 12),
                (&Event::PinkyBackward, 13),
                (&Event::ThrottleButtonL, 14),
                (&Event::EngineFuelFlowL, 15),
                (&Event::EngineFuelFlowR, 416),
                (&Event::EngineLMotor, 417),
                (&Event::EngineRMotor, 418),
                (&Event::Apu, 419),
                (&Event::LandingGearSilence, 420),
                (&Event::FlapsUp, 421),
                (&Event::FlapsDown, 422),
                (&Event::Eac, 423),
                (&Event::RadarAltimeter, 424),
                (&Event::AutopilotToggle, 425),
                (&Event::AutopilotPath, 426),
                (&Event::AutopilotAlt, 427),
                (&Event::EngineLIgnition, 430),
                (&Event::EngineRIgnition, 431),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&|p| Event::Slew(p * 0.5 + 0.5), 6, None),
            ],
            triggers: &[
                (&|v| Event::MouseX(v * 2.0 - 1.0), 0, Some(1024), None),
                (&|v| Event::MouseY(v * 2.0 - 1.0), 1, Some(1024), None),
                (&|v| Event::ThrottleR(1.0 - v), 2, Some(16339), None),
                (&|v| Event::ThrottleL(1.0 - v), 5, Some(16339), None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::PovLeft(state) } else { Event::PovRight(state) }, 16),
                (&|neg, state| if neg { Event::PovUp(state) } else { Event::PovDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x04B3, 0x1001) => &CtlrDescriptor {
            name: "Dual Power 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x07D0, 0x0001) => &CtlrDescriptor {
            name: "Thrustmaster T Mini Wireless",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x08D0, 0x0001) => &CtlrDescriptor {
            name: "Thrustmaster Run N Drive Wireless",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x09D0, 0x0001) => &CtlrDescriptor {
            name: "Thrustmaster Run N Drive Wireless PS3",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x0ED0, 0x1101) => &CtlrDescriptor {
            name: "ThrustMaster eSwap PRO Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x12B3, 0x1001) => &CtlrDescriptor {
            name: "Thrustmaster vibrating gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x15B3, 0x1001) => &CtlrDescriptor {
            name: "Thrustmaster Dual Analog 4",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x20B3, 0x1001) => &CtlrDescriptor {
            name: "Thrustmaster 2 in 1 DT",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x23B3, 0x0001) => &CtlrDescriptor {
            name: "Thrustmaster Dual Trigger 3-in-1",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x4F04, 0x26B3, 0x0204) => &CtlrDescriptor {
            name: "Thrustmaster Gamepad GP XID",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5509, 0x1072, 0x1101) => &CtlrDescriptor {
            name: "NVIDIA Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 7),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
                (&Event::Home, 13),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5509, 0x1472, 0x1101) => &CtlrDescriptor {
            name: "NVIDIA Controller v01.04",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 6),
                (&Event::JoyPush, 7),
                (&Event::CamPush, 8),
                (&Event::Prev, 14),
                (&Event::Home, 17),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x0202, 0x0001) => &CtlrDescriptor {
            name: "Old Xbox pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::BumperR, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x0E00, 0x0001) => &CtlrDescriptor {
            name: "Microsoft SideWinder",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 8),
                (&Event::Prev, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x1907, 0x0001) => &CtlrDescriptor {
            name: "X360 Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::DpadLeft, 11),
                (&Event::DpadRight, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8502, 0x0001) => &CtlrDescriptor {
            name: "Microsoft X-Box pad (Japan)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::BumperR, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8902, 0x2101) => &CtlrDescriptor {
            name: "Microsoft X-Box pad v2 (US)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::BumperR, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x0001) => &CtlrDescriptor {
            name: "xbox360 Wireless EasySMX",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x0401) => &CtlrDescriptor {
            name: "Microsoft X-Box 360 pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x1001) => &CtlrDescriptor {
            name: "X360 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::ActionB, 16),
                (&Event::ActionA, 17),
                (&Event::ActionH, 19),
                (&Event::ActionV, 20),
                (&Event::BumperL, 22),
                (&Event::BumperR, 23),
                (&Event::Prev, 26),
                (&Event::Next, 27),
                (&Event::Home, 28),
                (&Event::JoyPush, 29),
                (&Event::CamPush, 30),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, Some(0.992)),
                (&Event::JoyY, 1, Some(0.992)),
                (&Event::CamX, 3, Some(0.992)),
                (&Event::CamY, 4, Some(0.992)),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x1401) => &CtlrDescriptor {
            name: "X360 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x2001) => &CtlrDescriptor {
            name: "8BitDo Wireless Adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x2020) => &CtlrDescriptor {
            name: "SpeedLink XEOX Pro Analog Gamepad pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x6223) => &CtlrDescriptor {
            name: "Microsoft X-Box 360 pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x7005) => &CtlrDescriptor {
            name: "Torid",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x8E02, 0x7305) => &CtlrDescriptor {
            name: "Speedlink TORID Wireless Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0x9102, 0x0701) => &CtlrDescriptor {
            name: "X360 Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::DpadLeft, 11),
                (&Event::DpadRight, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xA102, 0x0001) => &CtlrDescriptor {
            name: "X360 Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::DpadLeft, 11),
                (&Event::DpadRight, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xA102, 0x0701) => &CtlrDescriptor {
            name: "X360 Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xA102, 0x1401) => &CtlrDescriptor {
            name: "Xbox 360 Wireless Receiver (XBOX)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::DpadLeft, 11),
                (&Event::DpadRight, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xD102, 0x0101) => &CtlrDescriptor {
            name: "Microsoft X-Box One pad",
            deadzone: None,
            buttons: &[
                (&Event::Prev, 6),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::ActionA, 16),
                (&Event::ActionB, 17),
                (&Event::ActionH, 19),
                (&Event::ActionV, 20),
                (&Event::BumperL, 22),
                (&Event::BumperR, 23),
                (&Event::Home, 26),
                (&Event::Next, 27),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xD102, 0x0201) => &CtlrDescriptor {
            name: "Xbox One Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xD102, 0x0302) => &CtlrDescriptor {
            name: "Microsoft X-Box One pad v2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xDD02, 0x0302) => &CtlrDescriptor {
            name: "Microsoft X-Box One pad (Firmware 2015)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xE302, 0x0302) => &CtlrDescriptor {
            name: "Microsoft X-Box One Elite pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xEA02, 0x0000) => &CtlrDescriptor {
            name: "Xbox One Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x5E04, 0xEA02, 0x0103) => &CtlrDescriptor {
            name: "Xbox One Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6325, 0x2305, 0x1001) => &CtlrDescriptor {
            name: "ShanWan USB Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6325, 0x2605, 0x1001) => &CtlrDescriptor {
            name: "HJD-X",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::Home, 12),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 4, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6325, 0x7505, 0x1001) => &CtlrDescriptor {
            name: "SHANWAN PS3/PC Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6666, 0x0488, 0x0001) => &CtlrDescriptor {
            name: "Super Joy Box 5 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 8),
                (&Event::Prev, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6666, 0x6706, 0x0001) => &CtlrDescriptor {
            name: "boom PSX to PC Converter",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::Next, 11),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6B14, 0x010C, 0x1001) => &CtlrDescriptor {
            name: "NACON GC-400ES",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6B14, 0x010D, 0x1101) => &CtlrDescriptor {
            name: "Revolution Pro Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x0AC2, 0x1001) => &CtlrDescriptor {
            name: "Logitech Inc. WingMan RumblePad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
            ],
            trigbtns: &[
                (&Event::TriggerR, 2),
                (&Event::TriggerL, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x11C2, 0x1001) => &CtlrDescriptor {
            name: "Logitech WingMan Cordless RumblePad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Prev, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::Home, 5),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 8),
            ],
            trigbtns: &[
                (&Event::TriggerL, 9),
                (&Event::TriggerR, 10),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x15C2, 0x1001) => &CtlrDescriptor {
            name: "Logitech Logitech Extreme 3D",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 2),
                (&Event::ActionB, 4),
                (&Event::ActionV, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::BumperL, 9),
                (&Event::BumperR, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::JoyPush(state) } else { Event::CamPush(state) }, 16),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x16C2, _) => &CtlrDescriptor {
            name: "Logitech Dual Action PlayStation Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::ActionH, 16),
                (&Event::ActionA, 17),
                (&Event::BumperL, 22),
                (&Event::BumperR, 23),
                (&Event::JoyPush, 26),
                (&Event::CamPush, 27),
                (&Event::Home, 28),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
                (&Event::TriggerL, 24),
                (&Event::TriggerR, 25),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
                (&Event::CamX, 5, Some(0.67)),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x18C2, 0x1001) => &CtlrDescriptor {
            name: "Logitech RumblePad 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x19C2, 0x1001) => &CtlrDescriptor {
            name: "Logitech Cordless RumblePad 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x19C2, 0x1101) => &CtlrDescriptor {
            name: "Logitech F710 Gamepad (DInput)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x1DC2, 0x1440) => &CtlrDescriptor {
            name: "Logitech F310 Gamepad (XInput)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x1EC2, 0x1920) => &CtlrDescriptor {
            name: "Logitech F510 Gamepad (XInput)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x1EC2, 0x2020) => &CtlrDescriptor {
            name: "Logitech F510 Gamepad (XInput)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0x1FC2, 0x0503) => &CtlrDescriptor {
            name: "Logitech F710 Gamepad (XInput)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6D04, 0xD2CA, 0x1101) => &CtlrDescriptor {
            name: "Precision Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6E05, 0x0320, 0x1001) => &CtlrDescriptor {
            name: "JC-U3613M - DirectInput Mode",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::ActionA, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x0103, 0x0002) => &CtlrDescriptor {
            name: "Logic3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x0104, 0x0001) => &CtlrDescriptor {
            name: "Gamestop Logic3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x0105, 0x0001) => &CtlrDescriptor {
            name: "PDP Wired Gamepad for Xbox 360",
            deadzone: Some(0.07),
            buttons: &[
                (&Event::ActionA, 16),
                (&Event::ActionB, 17),
                (&Event::ActionH, 19),
                (&Event::ActionV, 20),
                (&Event::BumperL, 22),
                (&Event::BumperR, 23),
                (&Event::Prev, 26),
                (&Event::Next, 27),
                (&Event::Home, 28),
                (&Event::JoyPush, 29),
                (&Event::CamPush, 30),
            ],
            trigbtns: &[
                (&Event::TriggerL, 24),
                (&Event::TriggerR, 25),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x0302, 0x1101) => &CtlrDescriptor {
            name: "Victrix Pro Fight Stick for PS4",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x0702, 0x1101) => &CtlrDescriptor {
            name: "Victrix Pro Fight Stick for PS4",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x0901, 0x1101) => &CtlrDescriptor {
            name: "PDP Versus Fighting Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x1302, 0x0001) => &CtlrDescriptor {
            name: "Afterglow",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x1304, 0x0001) => &CtlrDescriptor {
            name: "Generic X-Box pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x1402, 0x1101) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x1E01, 0x1101) => &CtlrDescriptor {
            name: "Rock Candy PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x1F01, 0x0001) => &CtlrDescriptor {
            name: "Rock Candy",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x3001, 0x0101) => &CtlrDescriptor {
            name: "EA Sports PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x3101, 0x0001) => &CtlrDescriptor {
            name: "PDP EA Sports Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x3901, 0x0043) => &CtlrDescriptor {
            name: "Afterglow Prismatic Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x3901, 0x1302) => &CtlrDescriptor {
            name: "Afterglow Prismatic Wired Controller 048-007-NA",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x3901, 0x2006) => &CtlrDescriptor {
            name: "Afterglow Controller for Xbox One",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x4601, 0x0101) => &CtlrDescriptor {
            name: "Rock Candy Xbox One Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0x6401, 0x0101) => &CtlrDescriptor {
            name: "PDP Battlefield One",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0xA802, _) => &CtlrDescriptor {
            name: "PDP Wired Controller for Xbox One",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, Some(63), None),
                (&Event::TriggerR, 5, Some(63), None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x6F0E, 0xC802, 0x1201) => &CtlrDescriptor {
            name: "PDP Kingdom Hearts Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7800, 0x0600, 0x1001) => &CtlrDescriptor {
            name: "Microntek USB Joystick",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0x0600, 0x0701) => &CtlrDescriptor {
            name: "USB Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0x0600, 0x1001) => &CtlrDescriptor {
            name: "DragonRise Inc. Generic USB Joystick",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0x1100, 0x0001) => &CtlrDescriptor {
            name: "USB Gamepad1",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadDown(state) } else { Event::DpadDown(state) }, 0),
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadLeft(state) }, 1),
                (&|neg, state| if neg { Event::DpadRight(state) } else { Event::DpadRight(state) }, 2),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadUp(state) }, 4),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0x1100, 0x1001) => &CtlrDescriptor {
            name: "Retrolink SNES Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0x4318, 0x1001) => &CtlrDescriptor {
            name: "Nintendo GameCube Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperR, 7),
                (&Event::Next, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0x4418, 0x1001) => &CtlrDescriptor {
            name: "GameCube Controller - Mayflash Adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperR, 7),
                (&Event::Next, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, Some(0.67)),
                (&Event::JoyY, 1, Some(0.67)),
                (&Event::CamY, 2, Some(0.67)),
                (&Event::CamX, 5, Some(0.67)),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, Some(0.125)),
                (&Event::TriggerR, 4, None, Some(0.125)),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0xD218, 0x1101) => &CtlrDescriptor {
            name: "Mayflash Magic NS",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7900, 0xD418, 0x0001) => &CtlrDescriptor {
            name: "GPD Win 2 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x791D, 0x0103, 0x1001) => &CtlrDescriptor {
            name: "Wii Classic Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7D04, 0x0540, 0x0001) => &CtlrDescriptor {
            name: "Gravis Eliminator GamePad Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7E05, 0x0620, 0x0100) => &CtlrDescriptor {
            name: "Joy-Con (L)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 8),
                (&Event::JoyPush, 10),
                (&Event::Prev, 13),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 16, None),
                (&Event::JoyY, 17, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7E05, 0x0720, 0x0100) => &CtlrDescriptor {
            name: "Joy-Con (R)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 9),
                (&Event::JoyPush, 11),
                (&Event::Prev, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 16, None),
                (&Event::JoyY, 17, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x7E05, 0x0920, 0x1181) => &CtlrDescriptor {
            name: "Nintendo Switch Pro Controller Wired (joycond)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 5),
                (&Event::BumperR, 6),
                (&Event::Prev, 9),
                (&Event::Next, 10),
                (&Event::Home, 11),
                (&Event::JoyPush, 12),
                (&Event::CamPush, 13),
            ],
            trigbtns: &[
                (&Event::TriggerL, 7),
                (&Event::TriggerR, 8),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8117, 0x990A, 0x0101) => &CtlrDescriptor {
            name: "Retronic Adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8305, 0x5020, 0x1001) => &CtlrDescriptor {
            name: "Padix Co. Ltd. Rockfire PSX/USB Bridge",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8305, 0x6020, 0x1001) => &CtlrDescriptor {
            name: "iBuffalo SNES Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8916, 0x00FD, 0x2401) => &CtlrDescriptor {
            name: "Razer Onza Tournament Edition",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::DpadLeft, 11),
                (&Event::DpadRight, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8916, 0x00FE, 0x2401) => &CtlrDescriptor {
            name: "Razer Sabertooth",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8916, 0x01FD, 0x2401) => &CtlrDescriptor {
            name: "Razer Onza Classic Edition",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::DpadLeft, 11),
                (&Event::DpadRight, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8F0E, 0x0300, 0x1001) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8F0E, 0x0610, 0x0001) => &CtlrDescriptor {
            name: "GreenAsia Electronics 4Axes 12Keys Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8F0E, 0x0800, 0x1001) => &CtlrDescriptor {
            name: "Gasia Co. Ltd PS(R) Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8F0E, 0x0D31, 0x1001) => &CtlrDescriptor {
            name: "SZMY-POWER CO. LTD. GAMEPAD 3 TURBO",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8F0E, 0x1200, 0x1001) => &CtlrDescriptor {
            name: "GreenAsia Inc. USB Joystick",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x8F0E, 0x1330, 0x1001) => &CtlrDescriptor {
            name: "HuiJia SNES Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x9B28, 0x0300, 0x0101) => &CtlrDescriptor {
            name: "raphnet.net 4nes4snes v1.5",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::Prev, 2),
                (&Event::Next, 3),
                (&Event::ActionB, 4),
                (&Event::ActionV, 5),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x9B28, 0x3200, 0x0101) => &CtlrDescriptor {
            name: "Raphnet Technologies GC/N64 to USB v3.4",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::BumperR, 2),
                (&Event::Next, 3),
                (&Event::ActionB, 7),
                (&Event::ActionV, 8),
                (&Event::DpadUp, 10),
                (&Event::DpadDown, 11),
                (&Event::DpadLeft, 12),
                (&Event::DpadRight, 13),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0x9B28, 0x6000, 0x0101) => &CtlrDescriptor {
            name: "Raphnet Technologies GC/N64 to USB v3.6",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::BumperR, 2),
                (&Event::Next, 3),
                (&Event::ActionB, 7),
                (&Event::ActionV, 8),
                (&Event::DpadUp, 10),
                (&Event::DpadDown, 11),
                (&Event::DpadLeft, 12),
                (&Event::DpadRight, 13),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x0901, 0x0001) => &CtlrDescriptor {
            name: "Saitek P880",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::ActionA, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x0B04, 0x0001) => &CtlrDescriptor {
            name: "Saitek P990 Dual Analog Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 8),
                (&Event::Prev, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x0C04, 0x1101) => &CtlrDescriptor {
            name: "Saitek P2900 Wireless Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Home, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Next, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x0CFF, 0x1001) => &CtlrDescriptor {
            name: "Saitek P2500 Force Rumble Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::ActionA, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
                (&Event::Home, 10),
                (&Event::Prev, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x1005, 0x0001) => &CtlrDescriptor {
            name: "Saitek Saitek P150",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::BumperR, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 7),
            ],
            trigbtns: &[
                (&Event::TriggerR, 5),
                (&Event::TriggerL, 6),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x18F5, 0x1001) => &CtlrDescriptor {
            name: "Saitek PLC Saitek P3200 Rumble Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x20F6, 0x1101) => &CtlrDescriptor {
            name: "Saitek PS2700 Rumble Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x22F6, 0x1101) => &CtlrDescriptor {
            name: "Cyborg V.3 Rumble Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
                (&|neg, state| if neg { Event::TriggerR(state) } else { Event::TriggerL(state) }, 3),
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xA306, 0x23F6, 0x1101) => &CtlrDescriptor {
            name: "Saitek Cyborg V.1 Game Pad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xAC05, 0x5B05, 0x1001) => &CtlrDescriptor {
            name: "Xiaoji Gamesir-G3w",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xAD1B, 0x01F5, 0x3305) => &CtlrDescriptor {
            name: "Hori Pad EX Turbo 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xAD1B, 0x16F0, 0x9004) => &CtlrDescriptor {
            name: "Mad Catz Xbox 360 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xAD1B, 0x2EF0, 0x9004) => &CtlrDescriptor {
            name: "Mad Catz Fightpad SFxT",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xAD1B, 0x38F0, 0x9004) => &CtlrDescriptor {
            name: "Street Fighter IV FightStick TE",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xB404, 0x0A01, 0x0001) => &CtlrDescriptor {
            name: "CYPRESS USB Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::Prev, 5),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 8),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xB507, 0x0399, 0x0001) => &CtlrDescriptor {
            name: "Thrustmaster Firestorm Digital 2",
            deadzone: None,
            buttons: &[
                (&Event::CamPush, 0),
                (&Event::Next, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::ActionB, 4),
                (&Event::ActionV, 5),
                (&Event::BumperL, 6),
                (&Event::BumperR, 8),
                (&Event::JoyPush, 10),
                (&Event::Prev, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 7),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xB507, 0x1503, 0x1001) => &CtlrDescriptor {
            name: "impact",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::ActionA, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 5),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xB507, 0x1603, 0x1001) => &CtlrDescriptor {
            name: "Thrustmaster Flightstick",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionV, 1),
                (&Event::ActionB, 2),
                (&Event::ActionH, 3),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&|p| Event::Throttle(p * 0.5 + 0.5), 2, Some(-1.0)),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::PovLeft(state) } else { Event::PovRight(state) }, 16),
                (&|neg, state| if neg { Event::PovUp(state) } else { Event::PovDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xBA22, 0x2010, 0x0101) => &CtlrDescriptor {
            name: "Jess Technology USB Game Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xBC20, 0x0055, 0x1001) => &CtlrDescriptor {
            name: "ShanWan PS3/PC Wired GamePad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xBC20, 0x0055, 0x1101) => &CtlrDescriptor {
            name: "GameSir G3w",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xBD12, 0x15D0, 0x1001) => &CtlrDescriptor {
            name: "Tomee SNES USB Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC011, 0x0140, 0x1101) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC011, 0x0591, 0x1101) => &CtlrDescriptor {
            name: "Torid",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC016, 0x8704, 0x1101) => &CtlrDescriptor {
            name: "Serial/Keyboard/Mouse/Joystick",
            deadzone: None,
            buttons: &[
                (&Event::DpadUp, 0),
                (&Event::DpadRight, 1),
                (&Event::DpadDown, 2),
                (&Event::DpadLeft, 3),
                (&Event::Prev, 4),
                (&Event::Next, 5),
                (&Event::BumperR, 8),
                (&Event::BumperL, 9),
                (&Event::ActionB, 10),
                (&Event::ActionV, 11),
                (&Event::ActionA, 12),
                (&Event::ActionH, 13),
                (&Event::JoyPush, 14),
                (&Event::CamPush, 15),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyY, 0, None),
                (&Event::JoyX, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC016, 0xE105, 0x0101) => &CtlrDescriptor {
            name: "Xin-Mo Xin-Mo Dual Arcade",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionH, 1),
                (&Event::BumperL, 2),
                (&Event::ActionB, 3),
                (&Event::ActionA, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 9),
                (&Event::DpadUp, 11),
                (&Event::DpadDown, 12),
                (&Event::DpadLeft, 13),
                (&Event::DpadRight, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC624, 0x0053, 0x0001) => &CtlrDescriptor {
            name: "PowerA",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC624, 0x025B, 0x0202) => &CtlrDescriptor {
            name: "Thrustmaster GPX Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC624, 0x045D, 0x2401) => &CtlrDescriptor {
            name: "Razer Sabertooth",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC624, 0x045D, 0x2501) => &CtlrDescriptor {
            name: "Razer Sabertooth",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC624, 0x1A53, 0x0001) => &CtlrDescriptor {
            name: "Mini PE",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC624, 0x2B89, 0x1101) => &CtlrDescriptor {
            name: "MOGA XP5-A Plus",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::Home, 12),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC624, 0x3A54, 0x0101) => &CtlrDescriptor {
            name: "PowerA 1428124-01",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0031, 0x1101) => &CtlrDescriptor {
            name: "8BitDo Wireless Adapter",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0090, 0x1101) => &CtlrDescriptor {
            name: "8BitDo FC30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 4, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0160, 0x0000) => &CtlrDescriptor {
            name: "8BitDo SN30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0160, 0x1101) => &CtlrDescriptor {
            name: "8BitDo SN30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 4, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0161, 0x0000) => &CtlrDescriptor {
            name: "8BitDo SN30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0190, 0x1101) => &CtlrDescriptor {
            name: "8BitDo NES30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 4, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0260, 0x1101) => &CtlrDescriptor {
            name: "8BitDo SN30 Pro+",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0310, 0x1101) => &CtlrDescriptor {
            name: "8BitDo NES30",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 7),
                (&Event::BumperR, 9),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 8),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x0650, 0x1101) => &CtlrDescriptor {
            name: "8BitDo M30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
                (&Event::TriggerL, 4, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x1290, 0x1101) => &CtlrDescriptor {
            name: "8Bitdo SN30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x1590, 0x1101) => &CtlrDescriptor {
            name: "8BitDo N30 Pro 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 4, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x1890, 0x1101) => &CtlrDescriptor {
            name: "8BitDo Zero 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC82D, 0x21AB, 0x1001) => &CtlrDescriptor {
            name: "8BitDo SFC30",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xC911, 0xF055, 0x1101) => &CtlrDescriptor {
            name: "HJC Game GAMEPAD",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD118, 0x0094, 0x1101) => &CtlrDescriptor {
            name: "Stadia Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD620, 0x10A7, 0x1101) => &CtlrDescriptor {
            name: "Mayflash Magic NS",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD620, 0x2A79, 0x1101) => &CtlrDescriptor {
            name: "BDA PS4 Fightpad",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD620, 0x6DCA, 0x1101) => &CtlrDescriptor {
            name: "PowerA Pro Ex",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD804, 0x8200, 0x0300) => &CtlrDescriptor {
            name: "IMS PCU#0 Gamepad Interface",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::Prev, 4),
                (&Event::Next, 5),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD814, 0x07CD, 0x1101) => &CtlrDescriptor {
            name: "Toodles 2008 Chimp PC/PS3",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD814, 0x0862, 0x1101) => &CtlrDescriptor {
            name: "HitBox (PS3/PC) Analog Mode",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Home, 9),
                (&Event::Next, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xD81D, 0x0E00, 0x1001) => &CtlrDescriptor {
            name: "Savior",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::BumperR, 2),
                (&Event::ActionH, 4),
                (&Event::ActionV, 5),
                (&Event::BumperL, 6),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerR, 3),
                (&Event::TriggerL, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xDE28, 0x0112, 0x0100) => &CtlrDescriptor {
            name: "Steam Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 3, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xDE28, 0x0211, 0x0100) => &CtlrDescriptor {
            name: "Steam Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 3, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xDE28, 0xFC11, 0x0100) => &CtlrDescriptor {
            name: "Steam Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xDE28, 0xFF11, 0x0100) => &CtlrDescriptor {
            name: "Steam Virtual Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xE820, 0x6058, 0x0101) => &CtlrDescriptor {
            name: "Cideko AK08b",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xF025, 0x21C1, 0x1001) => &CtlrDescriptor {
            name: "ShanWan Gioteck PS3 Wired Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xF025, 0xC183, 0x1001) => &CtlrDescriptor {
            name: "Goodbetterbest Ltd USB Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xF025, 0xC383, 0x1001) => &CtlrDescriptor {
            name: "GT VX2",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xFD05, 0x0030, 0x0001) => &CtlrDescriptor {
            name: "InterAct GoPad I-73000 (Fighting Game Layout)",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionV, 1),
                (&Event::BumperR, 2),
                (&Event::ActionA, 3),
                (&Event::ActionB, 4),
                (&Event::Prev, 6),
                (&Event::Next, 7),
            ],
            trigbtns: &[
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xFF11, 0x3133, 0x1001) => &CtlrDescriptor {
            name: "PC Game Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xFF11, 0x4133, 0x1001) => &CtlrDescriptor {
            name: "PS2 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionV, 0),
                (&Event::ActionB, 1),
                (&Event::ActionA, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0300, 0xFFFF, 0xFFFF, 0x0001) => &CtlrDescriptor {
            name: "Chinese-made Xbox Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::BumperR, 2),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x0100, 0x0100, 0x0300) => &CtlrDescriptor {
            name: "Nintendo Wiimote",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x050B, 0x0045, 0x3100) => &CtlrDescriptor {
            name: "ASUS Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Home, 6),
                (&Event::JoyPush, 7),
                (&Event::CamPush, 8),
                (&Event::Prev, 9),
                (&Event::Next, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x050B, 0x0045, 0x4000) => &CtlrDescriptor {
            name: "ASUS Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Home, 6),
                (&Event::JoyPush, 7),
                (&Event::CamPush, 8),
                (&Event::Prev, 9),
                (&Event::Next, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x1028, 0x0900, 0x0001) => &CtlrDescriptor {
            name: "8BitDo SFC30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x1101, 0x1914, 0x0901) => &CtlrDescriptor {
            name: "SteelSeries Stratus XL",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x1101, 0x3114, 0x1B01) => &CtlrDescriptor {
            name: "SteelSeries Stratus Duo",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Home, 32),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x1727, 0x4431, 0x2901) => &CtlrDescriptor {
            name: "XiaoMi Game Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Home, 20),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerR, 6, None, None),
                (&Event::TriggerL, 7, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x2028, 0x0900, 0x0001) => &CtlrDescriptor {
            name: "8BitDo SNES30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::DpadUp, 117),
                (&Event::DpadLeft, 119),
                (&Event::DpadRight, 120),
                (&Event::DpadDown, 122),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x2038, 0x0900, 0x0001) => &CtlrDescriptor {
            name: "8BitDo NES30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x3215, 0x0009, 0x163A) => &CtlrDescriptor {
            name: "Razer Serval",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x3628, 0x0100, 0x0201) => &CtlrDescriptor {
            name: "OUYA Game Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionV, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::JoyPush, 6),
                (&Event::CamPush, 7),
                (&Event::DpadUp, 8),
                (&Event::DpadDown, 9),
                (&Event::DpadLeft, 10),
                (&Event::DpadRight, 11),
                (&Event::Home, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x3628, 0x0100, 0x0301) => &CtlrDescriptor {
            name: "OUYA Game Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionH, 1),
                (&Event::ActionV, 2),
                (&Event::ActionB, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::JoyPush, 6),
                (&Event::CamPush, 7),
                (&Event::DpadUp, 8),
                (&Event::DpadDown, 9),
                (&Event::DpadLeft, 10),
                (&Event::DpadRight, 11),
                (&Event::Home, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x3807, 0x6652, 0x2501) => &CtlrDescriptor {
            name: "Mad Catz C.T.R.L.R ",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4919, 0x0204, 0x1B01) => &CtlrDescriptor {
            name: "Ipega PG-9069 - Bluetooth Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Home, 161),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4919, 0x0204, 0x2100) => &CtlrDescriptor {
            name: "Amazon Fire Game Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::Home, 12),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0x6802, 0x0001) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::Prev, 0),
                (&Event::JoyPush, 1),
                (&Event::CamPush, 2),
                (&Event::Next, 3),
                (&Event::DpadUp, 4),
                (&Event::DpadRight, 5),
                (&Event::DpadDown, 6),
                (&Event::DpadLeft, 7),
                (&Event::BumperL, 10),
                (&Event::BumperR, 11),
                (&Event::ActionV, 12),
                (&Event::ActionB, 13),
                (&Event::ActionA, 14),
                (&Event::ActionH, 15),
                (&Event::Home, 16),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerL, 12, None, None),
                (&Event::TriggerR, 13, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0x6802, 0x0080) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::DpadRight, 16),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0x6802, 0x0081) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::DpadRight, 16),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0xC405, 0x0001) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0xC405, 0x0081) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0xCC09, 0x0001) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionH, 0),
                (&Event::ActionA, 1),
                (&Event::ActionB, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0xCC09, 0x0081) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x4C05, 0xCC09, 0x0180) => &CtlrDescriptor {
            name: "PS4 Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x5509, 0x1472, 0x0100) => &CtlrDescriptor {
            name: "NVIDIA Controller v01.04",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 6),
                (&Event::JoyPush, 7),
                (&Event::CamPush, 8),
                (&Event::Prev, 14),
                (&Event::Home, 16),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x5E04, 0x050B, 0x0209) => &CtlrDescriptor {
            name: "Xbox One Elite Series 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Prev, 136),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 5, None, None),
                (&Event::TriggerL, 6, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x5E04, 0x050B, 0x0309) => &CtlrDescriptor {
            name: "Xbox One Elite Series 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 11),
                (&Event::Home, 12),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Prev, 121),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x5E04, 0xE002, 0x0309) => &CtlrDescriptor {
            name: "Xbox One Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::JoyPush, 8),
                (&Event::CamPush, 9),
                (&Event::Home, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x5E04, 0xFD02, 0x0309) => &CtlrDescriptor {
            name: "Xbox One Wireless Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Next, 11),
                (&Event::Home, 12),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Prev, 15),
                (&Event::Home, 16),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x5E04, 0xFD02, 0x3011) => &CtlrDescriptor {
            name: "Xbox One Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x7E05, 0x0620, 0x0100) => &CtlrDescriptor {
            name: "Joy-Con (L)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 8),
                (&Event::JoyPush, 10),
                (&Event::Prev, 13),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 16, None),
                (&Event::JoyY, 17, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x7E05, 0x0720, 0x0100) => &CtlrDescriptor {
            name: "Joy-Con (R)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 9),
                (&Event::JoyPush, 11),
                (&Event::Prev, 12),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 16, None),
                (&Event::JoyY, 17, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x7E05, 0x0920, 0x0100) => &CtlrDescriptor {
            name: "Nintendo Switch Pro Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
                (&Event::Home, 12),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x7E05, 0x0920, 0x0180) => &CtlrDescriptor {
            name: "Nintendo Switch Pro Controller (joycond)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 5),
                (&Event::BumperR, 6),
                (&Event::Prev, 9),
                (&Event::Next, 10),
                (&Event::Home, 11),
                (&Event::JoyPush, 12),
                (&Event::CamPush, 13),
            ],
            trigbtns: &[
                (&Event::TriggerL, 7),
                (&Event::TriggerR, 8),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0x7E05, 0x3003, 0x0100) => &CtlrDescriptor {
            name: "Nintendo Wii Remote Pro Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::Home, 10),
                (&Event::JoyPush, 11),
                (&Event::CamPush, 12),
                (&Event::DpadUp, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::DpadRight, 16),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 7),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xA005, 0x3232, 0x0100) => &CtlrDescriptor {
            name: "8BitDo Zero Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::DpadUp, 117),
                (&Event::DpadLeft, 119),
                (&Event::DpadRight, 120),
                (&Event::DpadDown, 122),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xA005, 0x3232, 0x0801) => &CtlrDescriptor {
            name: "8BitDo Zero Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 0),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 1),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xAC05, 0x3232, 0x0100) => &CtlrDescriptor {
            name: "VR-BOX",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 8),
                (&Event::Next, 9),
                (&Event::JoyPush, 10),
                (&Event::CamPush, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 4),
                (&Event::TriggerR, 5),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC624, 0x2A89, 0x0001) => &CtlrDescriptor {
            name: "MOGA XP5-A Plus",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
                (&Event::Home, 22),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x0060, 0x0001) => &CtlrDescriptor {
            name: "8BitDo SF30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x0061, 0x0001) => &CtlrDescriptor {
            name: "8BitDo SF30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x0161, 0x0001) => &CtlrDescriptor {
            name: "8BitDo SN30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionB, 0),
                (&Event::ActionA, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x0261, 0x0001) => &CtlrDescriptor {
            name: "8BitDo SN30 Pro+",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x1038, 0x0001) => &CtlrDescriptor {
            name: "8BitDo FC30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x2038, 0x0001) => &CtlrDescriptor {
            name: "8BitDo NES30 Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x3028, 0x0001) => &CtlrDescriptor {
            name: "8BitDo SFC30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x3032, 0x0001) => &CtlrDescriptor {
            name: "8BitDo Zero 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x5106, 0x0001) => &CtlrDescriptor {
            name: "8BitDo M30",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperR, 6),
                (&Event::BumperL, 8),
                (&Event::Next, 11),
            ],
            trigbtns: &[
                (&Event::TriggerR, 7),
                (&Event::TriggerL, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x6228, 0x0001) => &CtlrDescriptor {
            name: "8Bitdo SN30 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x6528, 0x0001) => &CtlrDescriptor {
            name: "8BitDo N30 Pro 2",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::Home, 2),
                (&Event::ActionV, 3),
                (&Event::ActionH, 4),
                (&Event::BumperL, 6),
                (&Event::BumperR, 7),
                (&Event::Prev, 10),
                (&Event::Next, 11),
                (&Event::JoyPush, 13),
                (&Event::CamPush, 14),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xC82D, 0x8010, 0x0001) => &CtlrDescriptor {
            name: "8BitDo NES30",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 3),
                (&Event::ActionV, 4),
                (&Event::BumperL, 7),
                (&Event::BumperR, 9),
                (&Event::Prev, 10),
                (&Event::Next, 11),
            ],
            trigbtns: &[
                (&Event::TriggerL, 6),
                (&Event::TriggerR, 8),
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xD620, 0x7162, 0x0100) => &CtlrDescriptor {
            name: "Moga Pro 2 HID",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 6),
                (&Event::JoyPush, 7),
                (&Event::CamPush, 8),
                (&Event::Prev, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xD620, 0xAD0D, 0x0100) => &CtlrDescriptor {
            name: "Moga Pro",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 6),
                (&Event::JoyPush, 7),
                (&Event::CamPush, 8),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xD620, 0xE589, 0x0100) => &CtlrDescriptor {
            name: "Moga 2 HID",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Next, 6),
                (&Event::JoyPush, 7),
                (&Event::CamPush, 8),
                (&Event::Prev, 9),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
                (&Event::TriggerR, 4, None, None),
                (&Event::TriggerL, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xDE28, 0x0212, 0x0100) => &CtlrDescriptor {
            name: "Steam Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 3, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xDE28, 0x0511, 0x0100) => &CtlrDescriptor {
            name: "Steam Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 3, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0500, 0xDE28, 0x0611, 0x0100) => &CtlrDescriptor {
            name: "Steam Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 3, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0600, 0x4C05, 0x6802, 0x0001) => &CtlrDescriptor {
            name: "PS3 Controller",
            deadzone: None,
            buttons: &[
                (&Event::Prev, 0),
                (&Event::JoyPush, 1),
                (&Event::CamPush, 2),
                (&Event::Next, 3),
                (&Event::DpadUp, 4),
                (&Event::DpadRight, 5),
                (&Event::DpadDown, 6),
                (&Event::DpadLeft, 7),
                (&Event::BumperL, 10),
                (&Event::BumperR, 11),
                (&Event::ActionV, 12),
                (&Event::ActionB, 13),
                (&Event::ActionA, 14),
                (&Event::ActionH, 15),
                (&Event::Home, 16),
            ],
            trigbtns: &[
                (&Event::TriggerL, 8),
                (&Event::TriggerR, 9),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0600, 0x7E05, 0x0820, 0x0000) => &CtlrDescriptor {
            name: "Nintendo Combined Joy-Cons (joycond)",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionV, 2),
                (&Event::ActionH, 3),
                (&Event::BumperL, 5),
                (&Event::BumperR, 6),
                (&Event::Prev, 9),
                (&Event::Next, 10),
                (&Event::JoyPush, 12),
                (&Event::CamPush, 13),
                (&Event::DpadUp, 14),
                (&Event::DpadDown, 15),
                (&Event::DpadLeft, 16),
                (&Event::DpadRight, 17),
            ],
            trigbtns: &[
                (&Event::TriggerL, 7),
                (&Event::TriggerR, 8),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 2, None),
                (&Event::CamY, 3, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (0x0600, 0xADDE, 0xEFBE, 0x0201) => &CtlrDescriptor {
            name: "Hidromancer Game Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::CamPush, 10),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamX, 3, None),
                (&Event::CamY, 4, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 5, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (_, 0x3807, 0x1817, _) => &CtlrDescriptor {
            name: "Mad Catz R.A.T. Pro X mouse",
            deadzone: None,
            buttons: &[
                (&Event::MouseMenu, 1),
                (&Event::WheelPush, 2),
                (&Event::Prev, 3),
                (&Event::Next, 4),
                (&Event::BumperL, 5),
                (&Event::Dpi, 6),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::MouseX, 0, None),
                (&Event::MouseY, 1, None),
                (&Event::WheelX, 5, None),
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
                (&Event::WheelY, 8),
            ],
        },
        (_, 0x4F04, 0x0AB1, _) => &CtlrDescriptor {
            name: "Thrustmaster T-16000M Flight Stick",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionV, 1),
                (&Event::ActionH, 2),
                (&Event::ActionB, 3),
                (&|p| Event::Action(0, p), 4),
                (&|p| Event::Action(1, p), 5),
                (&|p| Event::Action(2, p), 6),
                (&|p| Event::Action(3, p), 7),
                (&|p| Event::Action(4, p), 8),
                (&|p| Event::Action(5, p), 9),
                (&|p| Event::Action(6, p), 10),
                (&|p| Event::Action(7, p), 11),
                (&|p| Event::Action(8, p), 12),
                (&|p| Event::Action(9, p), 13),
                (&|p| Event::Action(10, p), 14),
                (&|p| Event::Action(11, p), 15),
                (&Event::DpadUp, 256),
                (&Event::DpadDown, 257),
                (&Event::DpadLeft, 258),
                (&Event::DpadRight, 259),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::JoyZ, 5, None),
                (&|p| Event::Throttle(p * 0.5 + 0.5), 6, None),
            ],
            triggers: &[
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (_, 0x6F0E, 0x0263, _) => &CtlrDescriptor {
            name: "Afterglow PlayStation3 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::ActionV, 16),
                (&Event::ActionA, 17),
                (&Event::ActionB, 18),
                (&Event::ActionH, 19),
                (&Event::BumperL, 20),
                (&Event::BumperR, 21),
                (&Event::Prev, 24),
                (&Event::Next, 25),
                (&Event::JoyPush, 26),
                (&Event::CamPush, 27),
                (&Event::Home, 28),
                (&Event::DpadUp, 256),
                (&Event::DpadDown, 257),
                (&Event::DpadLeft, 258),
                (&Event::DpadRight, 259),
            ],
            trigbtns: &[
                (&Event::TriggerL, 22),
                (&Event::TriggerR, 23),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (_, 0x7900, 0x3018, _) => &CtlrDescriptor {
            name: "Mayflash Arcade Fightstick F300 (PlayStation3/DInput Mode)",
            deadzone: None,
            buttons: &[
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::ActionV, 16),
                (&Event::ActionA, 17),
                (&Event::ActionB, 18),
                (&Event::ActionH, 19),
                (&Event::BumperL, 20),
                (&Event::BumperR, 21),
                (&Event::Prev, 24),
                (&Event::Next, 25),
                (&Event::JoyPush, 26),
                (&Event::CamPush, 27),
                (&Event::Home, 28),
                (&Event::DpadUp, 256),
                (&Event::DpadDown, 257),
                (&Event::DpadLeft, 258),
                (&Event::DpadRight, 259),
            ],
            trigbtns: &[
                (&Event::TriggerL, 22),
                (&Event::TriggerR, 23),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (_, 0x8F0E, 0x7530, _) => &CtlrDescriptor {
            name: "Speedlink PlayStation3 Gamepad",
            deadzone: None,
            buttons: &[
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::ActionV, 16),
                (&Event::ActionA, 17),
                (&Event::ActionB, 18),
                (&Event::ActionH, 19),
                (&Event::BumperL, 20),
                (&Event::BumperR, 21),
                (&Event::Prev, 24),
                (&Event::Next, 25),
                (&Event::JoyPush, 26),
                (&Event::CamPush, 27),
                (&Event::Home, 28),
                (&Event::DpadUp, 256),
                (&Event::DpadDown, 257),
                (&Event::DpadLeft, 258),
                (&Event::DpadRight, 259),
            ],
            trigbtns: &[
                (&Event::TriggerL, 22),
                (&Event::TriggerR, 23),
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
                (&Event::CamY, 2, None),
                (&Event::CamX, 5, None),
            ],
            triggers: &[
                (&Event::TriggerL, 3, None, None),
                (&Event::TriggerR, 4, None, None),
            ],
            three_ways: &[
                (&|neg, state| if neg { Event::DpadLeft(state) } else { Event::DpadRight(state) }, 16),
                (&|neg, state| if neg { Event::DpadUp(state) } else { Event::DpadDown(state) }, 17),
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (_, 0xDE28, 0x4211, _) => &CtlrDescriptor {
            name: "Steam Controller",
            deadzone: None,
            buttons: &[
                (&Event::ActionA, 0),
                (&Event::ActionB, 1),
                (&Event::ActionH, 2),
                (&Event::ActionV, 3),
                (&Event::BumperL, 4),
                (&Event::BumperR, 5),
                (&Event::Prev, 6),
                (&Event::Next, 7),
                (&Event::Home, 8),
                (&Event::JoyPush, 9),
                (&Event::DpadUp, 12),
                (&Event::DpadRight, 13),
                (&Event::DpadDown, 14),
                (&Event::DpadLeft, 15),
                (&Event::PaddleLeft, 48),
                (&Event::PaddleRight, 49),
            ],
            trigbtns: &[
            ],
            axes: &[
                (&Event::JoyX, 0, None),
                (&Event::JoyY, 1, None),
            ],
            triggers: &[
                (&Event::TriggerL, 2, None, None),
                (&Event::TriggerR, 3, None, None),
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
        (_, _, _, _) => &CtlrDescriptor {
            name: "Unknown Pad",
            deadzone: None,
            buttons: &[
            ],
            trigbtns: &[
            ],
            axes: &[
            ],
            triggers: &[
            ],
            three_ways: &[
            ],
            three_axes: &[
            ],
            wheels: &[
            ],
        },
    }
}
