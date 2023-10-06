use keyberon::action::{m, Action};
use keyberon::key_code::KeyCode::*;

const PL_LAYER: Action = Action::DefaultLayer(0);
const US_LAYER: Action = Action::DefaultLayer(3);

const TTY1: Action = m(&&[LCtrl, LAlt, F1].as_slice());
const TTY2: Action = m(&&[LCtrl, LAlt, F2].as_slice());

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4> = keyberon::layout::layout! {
    {
        [ Tab      Q       W       E       R       T       Y       U       I       O       P       BSpace ]
        [ Escape   A       S       D       F       G       H       J       K       L       ;       Quote  ]
        [ LShift   Z       X       C       V       B       N       M       ,       .       /       RShift ]
        [ t        t       t       LGui    Space   LCtrl   (1)     Enter   RAlt    t       t       t      ]
    }{
        [ t      1       2       3       4       5       6       7       8       9       0       t ]
        [ '`'    '['     ']'     '('     ')'     '\\'    |       =       +       -       '_'     ~ ]
        [ t      !       @       '{'     '}'     %       ^       &       *       #       $       t ]
        [ n      n       n       t       t       (2)     t       t       t       n       n       n ]
    }{
        [ n      F1             F2        F3            F4          F5      F6      F7      F8  F9      F10   Delete ]
        [ n      MediaPlayPause VolDown   Mute          VolUp       PgUp    Left    Down    Up  Right   F11   {TTY1} ]
        [ n      n              n         {US_LAYER}   {PL_LAYER}   PgDown   n      n       n   n       F12   {TTY2} ]
        [ n      n              n         n             n            n       n      n       n   n       n     n      ]
    }{
        [ Tab      Q       W       E       R       T       Y       U       I       O       P       BSpace ]
        [ Escape   A       S       D       F       G       H       J       K       L       ;       Quote  ]
        [ LShift   Z       X       C       V       B       N       M       ,       .       /       RShift ]
        [ t        t       t       LGui    Space   LCtrl   (1)     Enter   LAlt    t       t       t      ]
    }
};
