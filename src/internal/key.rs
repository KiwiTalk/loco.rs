

pub const LOCO_PEM_PUB_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIDANBgkqhkiG9w0BAQEFAAOCAQ0AMIIBCAKCAQEApElgRBx+g7sniYFW7LE8ivrwXShKTRFV8lXNItMXbN5QSC8vJ/\
cTSOTS619Xv5Zx7xXJIk4EKxtWesEGbgZpEUP2xQ+IeH9oz0JxayEMvvD1nVNAWgpWE4pociEoArsK7qY3YwXb1CiDHo9hojLv7djbo3cwXvlyMh4TUrX2RjCZPlVJxk/\
LVjzcl9ohJLkl3eoSrf0AE4kQ9mk3+raEhq5Dv+IDxKYX+fIytUWKmrQJusjtre9oVUX5sBOYZ0dzez/\
XapusEhUWImmB6mciVXfRXQ8IK4IH6vfNyxMSOTfLEhRYN2SMLzplAYFiMV536tLS3VmG5GJRdkpDubqPeQIBAw==
-----END PUBLIC KEY-----";

// Type check required
pub const LOCO_PUB_KEY: ([u8; 256], u8) = (
    hex_literal::hex!(
        "
        a449 6044 1c7e 83bb 2789 8156 ecb1 3c8a
        faf0 5d28 4a4d 1155 f255 cd22 d317 6cde
        5048 2f2f 27f7 1348 e4d2 eb5f 57bf 9671
        ef15 c922 4e04 2b1b 567a c106 6e06 6911
        43f6 c50f 8878 7f68 cf42 716b 210c bef0
        f59d 5340 5a0a 5613 8a68 7221 2802 bb0a
        eea6 3763 05db d428 831e 8f61 a232 efed
        d8db a377 305e f972 321e 1352 b5f6 4630
        993e 5549 c64f cb56 3cdc 97da 2124 b925
        ddea 12ad fd00 1389 10f6 6937 fab6 8486
        ae43 bfe2 03c4 a617 f9f2 32b5 458a 9ab4
        09ba c8ed adef 6855 45f9 b013 9867 4773
        7b3f d76a 9bac 1215 1622 6981 ea67 2255
        77d1 5d0f 082b 8207 eaf7 cdcb 1312 3937
        cb12 1458 3764 8c2f 3a65 0181 6231 5e77
        ead2 d2dd 5986 e462 5176 4a43 b9ba 8f79
        "
    ),
    0x03,
);