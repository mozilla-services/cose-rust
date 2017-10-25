#[cfg(test)]
use cose::test_setup as test;
#[cfg(test)]
use cose::cose;
#[cfg(test)]
use cose::decoder::CoseSignatureType;

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_rfc6979_test_vector_cose_1() {
    test::setup();
    let payload = b"This is the content.";
    let cose_signature =
        vec![0xD8, 0x62,
             0x84,
                 0x40,                                // bytes(0)
                                                      // ""
                 0xA0,                                // map(0)
                 0x54,                                // bytes(20)
                     0x54, 0x68, 0x69, 0x73, 0x20,    // "This is the content."
                     0x69, 0x73, 0x20, 0x74, 0x68,    // This is ignored here!
                     0x65, 0x20, 0x63, 0x6F, 0x6E,
                     0x74, 0x65, 0x6E, 0x74, 0x2E,
                 0x81,                                // array(1)
                     0x83,                            // array(3)
                         0x43,                        // bytes(3)
                             0xA1, 0x01, 0x26,        // "\xA1\x01&"
                         0xA1,                        // map(1)
                             0x04,                    // unsigned(4)
                             0x58, 0x5B,              // bytes(91)
                             0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02,
                             0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03,
                             0x42, 0x00, 0x04, 0xba, 0xc5, 0xb1, 0x1c, 0xad, 0x8f, 0x99, 0xf9, 0xc7,
                             0x2b, 0x05, 0xcf, 0x4b, 0x9e, 0x26, 0xd2, 0x44, 0xdc, 0x18, 0x9f, 0x74,
                             0x52, 0x28, 0x25, 0x5a, 0x21, 0x9a, 0x86, 0xd6, 0xa0, 0x9e, 0xff, 0x20,
                             0x13, 0x8b, 0xf8, 0x2d, 0xc1, 0xb6, 0xd5, 0x62, 0xbe, 0x0f, 0xa5, 0x4a,
                             0xb7, 0x80, 0x4a, 0x3a, 0x64, 0xb6, 0xd7, 0x2c, 0xcf, 0xed, 0x6b, 0x6f,
                             0xb6, 0xed, 0x28, 0xbb, 0xfc, 0x11, 0x7e, // RFC8152_KID11_TEST_SPKI
                         0x58, 0x40,                  // bytes(64)
                             0xe2, 0xae, 0xaf, 0xd4, 0x0d, 0x69, 0xd1, 0x9d, 0xfe, 0x6e, 0x52, 0x07,
                             0x7c, 0x5d, 0x7f, 0xf4, 0xe4, 0x08, 0x28, 0x2c, 0xbe, 0xfb, 0x5d, 0x06,
                             0xcb, 0xf4, 0x14, 0xaf, 0x2e, 0x19, 0xd9, 0x82, 0xac, 0x45, 0xac, 0x98,
                             0xb8, 0x54, 0x4c, 0x90, 0x8b, 0x45, 0x07, 0xde, 0x1e, 0x90, 0xb7, 0x17,
                             0xc3, 0xd3, 0x48, 0x16, 0xfe, 0x92, 0x6a, 0x2b, 0x98, 0xf5, 0x3a, 0xfd,
                             0x2f, 0xa0, 0xf3, 0x0a]; // signature bytes
    assert!(cose::verify_signature(payload, cose_signature).is_ok());
}

#[test]
fn test_cose_sign() {
    test::setup();
    let payload = b"This is the content.";
    let public_key = vec![0x04, 0xba, 0xc5, 0xb1, 0x1c, 0xad, 0x8f, 0x99, 0xf9,
                          0xc7, 0x2b, 0x05, 0xcf, 0x4b, 0x9e, 0x26, 0xd2, 0x44,
                          0xdc, 0x18, 0x9f, 0x74, 0x52, 0x28, 0x25, 0x5a, 0x21,
                          0x9a, 0x86, 0xd6, 0xa0, 0x9e, 0xff, 0x20, 0x13, 0x8b,
                          0xf8, 0x2d, 0xc1, 0xb6, 0xd5, 0x62, 0xbe, 0x0f, 0xa5,
                          0x4a, 0xb7, 0x80, 0x4a, 0x3a, 0x64, 0xb6, 0xd7, 0x2c,
                          0xcf, 0xed, 0x6b, 0x6f, 0xb6, 0xed, 0x28, 0xbb, 0xfc,
                          0x11, 0x7e];
    let secret_key = vec![0x57, 0xc9, 0x20, 0x77, 0x66, 0x41, 0x46, 0xe8, 0x76,
                          0x76, 0x0c, 0x95, 0x20, 0xd0, 0x54, 0xaa, 0x93, 0xc3,
                          0xaf, 0xb0, 0x4e, 0x30, 0x67, 0x05, 0xdb, 0x60, 0x90,
                          0x30, 0x85, 0x07, 0xb4, 0xd3];
    let cose_signature = cose::sign(payload, CoseSignatureType::ES256, &public_key, &secret_key);
    assert!(cose_signature.is_ok());
    let cose_signature = cose_signature.unwrap();

    // Verify signature.
    assert!(cose::verify_signature(payload, cose_signature).is_ok());
}
