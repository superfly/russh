#[allow(non_camel_case_types, missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Standard pseudo-terminal codes.
pub enum Pty {
    TTY_OP_END = 0,
    /// Interrupt character; 255 if none.
    VINTR = 1,
    /// The quit character (sends SIGQUIT signal on POSIX systems).
    VQUIT = 2,
    /// Erase the character to left of the cursor.
    VERASE = 3,
    /// Kill the current input line.
    VKILL = 4,
    /// End-of-file character (sends EOF from the terminal).
    VEOF = 5,
    /// End-of-line character in addition to carriage return and/or line-feed.
    VEOL = 6,
    /// Additional end-of-line character.
    VEOL2 = 7,
    /// Continues paused output (normally control-Q).
    VSTART = 8,
    /// Pauses output (normally control-S).
    VSTOP = 9,
    /// Suspends the current program.
    VSUSP = 10,
    /// Another suspend character.
    VDSUSP = 11,

    /// Reprints the current input line.
    VREPRINT = 12,
    /// Erases a word left of cursor.
    VWERASE = 13,
    /// Enter the next character typed literally, even if it is a special character
    VLNEXT = 14,
    /// Character to flush output.
    VFLUSH = 15,
    /// Switch to a different shell layer.
    VSWTCH = 16,
    /// Prints system status line (load, command, pid, etc).
    VSTATUS = 17,
    /// Toggles the flushing of terminal output.
    VDISCARD = 18,
    /// The ignore parity flag.
    IGNPAR = 30,
    /// Mark parity and framing errors.
    PARMRK = 31,
    /// Enable checking of parity errors.
    INPCK = 32,
    /// Strip 8th bit off characters.
    ISTRIP = 33,
    /// Map NL into CR on input.
    INLCR = 34,
    /// Ignore CR on input.
    IGNCR = 35,
    /// Map CR to NL on input.
    ICRNL = 36,
    /// Translate uppercase characters to lowercase.
    IUCLC = 37,
    /// Enable output flow control.
    IXON = 38,
    /// Any char will restart after stop.
    IXANY = 39,
    /// Enable input flow control.
    IXOFF = 40,
    /// Ring bell on input queue full.
    IMAXBEL = 41,
    /// I/O uses the UTF-8 character encoding
    IUTF8 = 42,
    /// Enable signals INTR, QUIT, [D]SUSP.
    ISIG = 50,
    /// Canonicalize input lines.
    ICANON = 51,
    /// Enable input and output of uppercase characters by preceding their lowercase equivalents with "\".
    XCASE = 52,
    /// Enable echoing.
    ECHO = 53,
    /// Visually erase chars.
    ECHOE = 54,
    /// Kill character discards current line.
    ECHOK = 55,
    /// Echo NL even if ECHO is off.
    ECHONL = 56,
    /// Don't flush after interrupt.
    NOFLSH = 57,
    /// Stop background jobs from output.
    TOSTOP = 58,
    /// Enable extensions.
    IEXTEN = 59,
    /// Echo control characters as ^(Char).
    ECHOCTL = 60,
    /// Visual erase for line kill.
    ECHOKE = 61,
    /// Retype pending input.
    PENDIN = 62,
    /// Enable output processing.
    OPOST = 70,
    /// Convert lowercase to uppercase.
    OLCUC = 71,
    /// Map NL to CR-NL.
    ONLCR = 72,
    /// Translate carriage return to newline (output).
    OCRNL = 73,
    /// Translate newline to carriage return-newline (output).
    ONOCR = 74,
    /// Newline performs a carriage return (output).
    ONLRET = 75,

    /// 7-bit mode
    CS7 = 90,
    /// 8-bit mode
    CS8 = 91,
    /// Parity enable.
    PARENB = 92,
    /// Odd parity, else even.
    PARODD = 93,

    /// Specifies the input baud rate in bits per second.
    TTY_OP_ISPEED = 128,
    /// Specifies the output baud rate in bits per second.
    TTY_OP_OSPEED = 129,
}

impl Pty {
    #[doc(hidden)]
    pub fn from_u8(x: u8) -> Option<Pty> {
        match x {
            0 => None,
            1 => Some(Pty::VINTR),
            2 => Some(Pty::VQUIT),
            3 => Some(Pty::VERASE),
            4 => Some(Pty::VKILL),
            5 => Some(Pty::VEOF),
            6 => Some(Pty::VEOL),
            7 => Some(Pty::VEOL2),
            8 => Some(Pty::VSTART),
            9 => Some(Pty::VSTOP),
            10 => Some(Pty::VSUSP),
            11 => Some(Pty::VDSUSP),

            12 => Some(Pty::VREPRINT),
            13 => Some(Pty::VWERASE),
            14 => Some(Pty::VLNEXT),
            15 => Some(Pty::VFLUSH),
            16 => Some(Pty::VSWTCH),
            17 => Some(Pty::VSTATUS),
            18 => Some(Pty::VDISCARD),
            30 => Some(Pty::IGNPAR),
            31 => Some(Pty::PARMRK),
            32 => Some(Pty::INPCK),
            33 => Some(Pty::ISTRIP),
            34 => Some(Pty::INLCR),
            35 => Some(Pty::IGNCR),
            36 => Some(Pty::ICRNL),
            37 => Some(Pty::IUCLC),
            38 => Some(Pty::IXON),
            39 => Some(Pty::IXANY),
            40 => Some(Pty::IXOFF),
            41 => Some(Pty::IMAXBEL),
            42 => Some(Pty::IUTF8),
            50 => Some(Pty::ISIG),
            51 => Some(Pty::ICANON),
            52 => Some(Pty::XCASE),
            53 => Some(Pty::ECHO),
            54 => Some(Pty::ECHOE),
            55 => Some(Pty::ECHOK),
            56 => Some(Pty::ECHONL),
            57 => Some(Pty::NOFLSH),
            58 => Some(Pty::TOSTOP),
            59 => Some(Pty::IEXTEN),
            60 => Some(Pty::ECHOCTL),
            61 => Some(Pty::ECHOKE),
            62 => Some(Pty::PENDIN),
            70 => Some(Pty::OPOST),
            71 => Some(Pty::OLCUC),
            72 => Some(Pty::ONLCR),
            73 => Some(Pty::OCRNL),
            74 => Some(Pty::ONOCR),
            75 => Some(Pty::ONLRET),

            90 => Some(Pty::CS7),
            91 => Some(Pty::CS8),
            92 => Some(Pty::PARENB),
            93 => Some(Pty::PARODD),

            128 => Some(Pty::TTY_OP_ISPEED),
            129 => Some(Pty::TTY_OP_OSPEED),
            _ => None,
        }
    }
}
