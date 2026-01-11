use keyberon::action::{Action::*, HoldTapAction, HoldTapConfig, k, m};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

macro_rules! hold_tap {
    ($hold:expr, $tap:expr) => {
        HoldTap(&HoldTapAction {
            timeout: 180,
            tap_hold_interval: 0,
            config: HoldTapConfig::Default,
            hold: $hold,
            tap: $tap,
        })
    };
}

macro_rules! s {
    ($key:expr) => {
        m(&[LShift, $key].as_slice())
    };
}

const CUT: Action = m(&[LShift, Delete].as_slice());
const COPY: Action = m(&[LCtrl, Insert].as_slice());
const PASTE: Action = m(&[LShift, Insert].as_slice());

const C_ESC: Action = hold_tap!(k(LCtrl), k(Escape));
const S_TAB: Action = hold_tap!(k(LShift), k(Tab));

const G_E: Action = hold_tap!(k(LGui), k(E));
const M_S: Action = hold_tap!(k(LAlt), k(S));
const C_D: Action = hold_tap!(k(LCtrl), k(D));
const S_F: Action = hold_tap!(k(LShift), k(F));
const S_J: Action = hold_tap!(k(RShift), k(J));
const C_K: Action = hold_tap!(k(RCtrl), k(K));
const M_L: Action = hold_tap!(k(LAlt), k(L));
const G_I: Action = hold_tap!(k(RGui), k(I));

const G_SH: Action = hold_tap!(k(LGui), s!(Kb3));
const M_2: Action = hold_tap!(k(LAlt), k(Kb2));
const C_3: Action = hold_tap!(k(LCtrl), k(Kb3));
const S_4: Action = hold_tap!(k(LShift), k(Kb4));
const S_7: Action = hold_tap!(k(RShift), k(Kb7));
const C_8: Action = hold_tap!(k(RCtrl), k(Kb8));
const M_9: Action = hold_tap!(k(LAlt), k(Kb9));
const G_AS: Action = hold_tap!(k(RGui), s!(Kb8));

const G_F3: Action = hold_tap!(k(LGui), k(F3));
const G_F8: Action = hold_tap!(k(LGui), k(F8));

const STAB: Action = m(&[LShift, Tab].as_slice());
const CBS: Action = m(&[LCtrl, BSpace].as_slice());

const COLON: Action = m(&[LShift, N].as_slice());
const EQUAL: Action = m(&[RAlt, G].as_slice());

pub const NUM_LAYERS: usize = 4;

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<10, 4, NUM_LAYERS, ()> = keyberon::layout::layout! {
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [   Q       W       E       R       T       Y       U       I       O       P   ],
        [   A       S       D       F       G       H       J       K       L       ;   ],
        [   Z       X       C       V       B       N       M       ,       .       /   ],
        [   n       n     LCtrl   Space    (1)     (2)   RShift   RAlt      n       n   ],
    }{//[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [ Pause CapsLock  LGui  PScreen ScrollLock  n    BSpace  Delete  Insert     n   ],
        [   n     LAlt   {C_ESC} {S_TAB} {STAB}   {CBS}   Left    Down     Up     Right ],
        [ Undo    {CUT}  {COPY}  {PASTE}    n     Enter   Home   PgDown   PgUp     End  ],
        [   n       n       t       t       n      (3)      t       t       n       n   ],
    }{//[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [   !       @     {G_SH}    $       %       ^       &     {G_AS}   '('     ')'  ],
        [   1     {M_2}   {C_3}   {S_4}     5       6     {S_7}   {C_8}   {M_9}     0   ],
        [   n       n    {COLON}    .    {EQUAL}    N  KpPlus KpMinus KpSlash KpAsterisk],
        [   n       n       t       t      (3)      n       t       t       n       n   ],
    }{//[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [  F1      F2     {G_F3}   F4      F5      F6      F7     {G_F8}   F9      F10  ],
        [   n     LAlt    LCtrl  LShift    '['     ']'   RShift   RCtrl   LAlt      n   ],
        [  F11     F12 NonUsBslash '`'      n       n     '\''    '\\'      -       =   ],
        [   n       n  {Custom(())} t       n       n       t       t       n       n   ],
    } //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
};
