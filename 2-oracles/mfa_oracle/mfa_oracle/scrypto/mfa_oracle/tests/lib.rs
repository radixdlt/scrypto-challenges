use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn test_check_before_auth() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, _account) = executor.new_account();
    let package = executor.publish_package(include_package!("testing")).unwrap();

    // Instantiate with virtual badge for auth
    let transaction1 = TransactionBuilder::new()
        .create_proof_from_auth_zone(ECDSA_TOKEN, |b, proof_id|
            b.call_function(package, "MFAOracle", "new_localhost", args![Proof(proof_id)])
        )
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());

    // Test the `free_token` method.
    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(component, "check", args![])
        .build(executor.get_nonce([pk]))
        .sign([]);
    //        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(!receipt2.result.is_ok()); // expect this to fila with "need MFA"
}

/*

#[test]
fn test_register() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, _account) = executor.new_account();
    let package = executor.publish_package(include_package!("testing")).unwrap();

    // Test the `instantiate_hello` function.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "MFAOracle", "new", args![])
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());

    // register_arg
    let register_arg = r#"{"id":"0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM","response":{"attestationObject":"o2NmbXRmcGFja2VkZ2F0dFN0bXSjY2FsZyZjc2lnWEcwRQIgL2u+F31GemAIm+yubtYN4FcHOobBzI6gzci+mGsmpwoCIQC3T+D3NvUZdnS3NnW7QrjQEnWzIYSwvb/eyYSOYz2sEWN4NWOBWQHeMIIB2jCCAX2gAwIBAgIBATANBgkqhkiG9w0BAQsFADBgMQswCQYDVQQGEwJVUzERMA8GA1UECgwIQ2hyb21pdW0xIjAgBgNVBAsMGUF1dGhlbnRpY2F0b3IgQXR0ZXN0YXRpb24xGjAYBgNVBAMMEUJhdGNoIENlcnRpZmljYXRlMB4XDTE3MDcxNDAyNDAwMFoXDTQyMDUxNjIyNTM0MVowYDELMAkGA1UEBhMCVVMxETAPBgNVBAoMCENocm9taXVtMSIwIAYDVQQLDBlBdXRoZW50aWNhdG9yIEF0dGVzdGF0aW9uMRowGAYDVQQDDBFCYXRjaCBDZXJ0aWZpY2F0ZTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABI1hfmXJUI5kvMVnOsgqZ5naPBRGaCwljEY//99Y39L6Pmw3i1PXlcSk3/tBme3Xhi8jq68CA7S4kRugVpmU4QGjJTAjMBMGCysGAQQBguUcAgEBBAQDAgUgMAwGA1UdEwEB/wQCMAAwDQYJKoZIhvcNAQELBQADSAAwRQIgB5BNIIoB+FUjTezB8P7zcmT6vjb5ip3J+tD0PpXiN7cCIQCBr/1ysP8G325/xJfv6XYlsjOEXFkV4tZ1lFjt0izgiWhhdXRoRGF0YVikSZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2NFAAAAAQECAwQFBgcIAQIDBAUGBwgAINIvfTCEHxlUirlmVbmeLqGZ7zumYmmFfHOnlphxDQ2DpQECAyYgASFYIALBcmd1ptldpytKngB8vYy2PD0fx9OnoVn0RE2CmrWCIlggUaLtszoKSVN3ECQnX/pJbWaKboXSLaSRjGLd5HFOcrM=","clientDataJSON":"eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiLXNnSXVSZ2dhbFkyRS1jLW1Mc3FJdU13Tkk5bHZhMDZYb25oR1BWVGRFRSIsIm9yaWdpbiI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MCIsImNyb3NzT3JpZ2luIjpmYWxzZX0="}}"#;

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(component, "register", args![register_arg])
        .build(executor.get_nonce([pk]))
        .sign([]);
    //        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

#[test]
fn test_validate() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, _account) = executor.new_account();
    let package = executor.publish_package(include_package!("testing")).unwrap();

    // Test the `instantiate_hello` function.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "MFAOracle", "new", args![])
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());

    // register before validate
    // register_arg
    let register_arg = r#"{"id":"0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM","response":{"attestationObject":"o2NmbXRmcGFja2VkZ2F0dFN0bXSjY2FsZyZjc2lnWEcwRQIgL2u+F31GemAIm+yubtYN4FcHOobBzI6gzci+mGsmpwoCIQC3T+D3NvUZdnS3NnW7QrjQEnWzIYSwvb/eyYSOYz2sEWN4NWOBWQHeMIIB2jCCAX2gAwIBAgIBATANBgkqhkiG9w0BAQsFADBgMQswCQYDVQQGEwJVUzERMA8GA1UECgwIQ2hyb21pdW0xIjAgBgNVBAsMGUF1dGhlbnRpY2F0b3IgQXR0ZXN0YXRpb24xGjAYBgNVBAMMEUJhdGNoIENlcnRpZmljYXRlMB4XDTE3MDcxNDAyNDAwMFoXDTQyMDUxNjIyNTM0MVowYDELMAkGA1UEBhMCVVMxETAPBgNVBAoMCENocm9taXVtMSIwIAYDVQQLDBlBdXRoZW50aWNhdG9yIEF0dGVzdGF0aW9uMRowGAYDVQQDDBFCYXRjaCBDZXJ0aWZpY2F0ZTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABI1hfmXJUI5kvMVnOsgqZ5naPBRGaCwljEY//99Y39L6Pmw3i1PXlcSk3/tBme3Xhi8jq68CA7S4kRugVpmU4QGjJTAjMBMGCysGAQQBguUcAgEBBAQDAgUgMAwGA1UdEwEB/wQCMAAwDQYJKoZIhvcNAQELBQADSAAwRQIgB5BNIIoB+FUjTezB8P7zcmT6vjb5ip3J+tD0PpXiN7cCIQCBr/1ysP8G325/xJfv6XYlsjOEXFkV4tZ1lFjt0izgiWhhdXRoRGF0YVikSZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2NFAAAAAQECAwQFBgcIAQIDBAUGBwgAINIvfTCEHxlUirlmVbmeLqGZ7zumYmmFfHOnlphxDQ2DpQECAyYgASFYIALBcmd1ptldpytKngB8vYy2PD0fx9OnoVn0RE2CmrWCIlggUaLtszoKSVN3ECQnX/pJbWaKboXSLaSRjGLd5HFOcrM=","clientDataJSON":"eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiLXNnSXVSZ2dhbFkyRS1jLW1Mc3FJdU13Tkk5bHZhMDZYb25oR1BWVGRFRSIsIm9yaWdpbiI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MCIsImNyb3NzT3JpZ2luIjpmYWxzZX0="}}"#;

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(component, "register", args![register_arg])
        .build(executor.get_nonce([pk]))
        .sign([]);
    //        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());

    //let register_arg = r#"{"id":"0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM","response":{"attestationObject":"o2NmbXRmcGFja2VkZ2F0dFN0bXSjY2FsZyZjc2lnWEcwRQIgL2u+F31GemAIm+yubtYN4FcHOobBzI6gzci+mGsmpwoCIQC3T+D3NvUZdnS3NnW7QrjQEnWzIYSwvb/eyYSOYz2sEWN4NWOBWQHeMIIB2jCCAX2gAwIBAgIBATANBgkqhkiG9w0BAQsFADBgMQswCQYDVQQGEwJVUzERMA8GA1UECgwIQ2hyb21pdW0xIjAgBgNVBAsMGUF1dGhlbnRpY2F0b3IgQXR0ZXN0YXRpb24xGjAYBgNVBAMMEUJhdGNoIENlcnRpZmljYXRlMB4XDTE3MDcxNDAyNDAwMFoXDTQyMDUxNjIyNTM0MVowYDELMAkGA1UEBhMCVVMxETAPBgNVBAoMCENocm9taXVtMSIwIAYDVQQLDBlBdXRoZW50aWNhdG9yIEF0dGVzdGF0aW9uMRowGAYDVQQDDBFCYXRjaCBDZXJ0aWZpY2F0ZTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABI1hfmXJUI5kvMVnOsgqZ5naPBRGaCwljEY//99Y39L6Pmw3i1PXlcSk3/tBme3Xhi8jq68CA7S4kRugVpmU4QGjJTAjMBMGCysGAQQBguUcAgEBBAQDAgUgMAwGA1UdEwEB/wQCMAAwDQYJKoZIhvcNAQELBQADSAAwRQIgB5BNIIoB+FUjTezB8P7zcmT6vjb5ip3J+tD0PpXiN7cCIQCBr/1ysP8G325/xJfv6XYlsjOEXFkV4tZ1lFjt0izgiWhhdXRoRGF0YVikSZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2NFAAAAAQECAwQFBgcIAQIDBAUGBwgAINIvfTCEHxlUirlmVbmeLqGZ7zumYmmFfHOnlphxDQ2DpQECAyYgASFYIALBcmd1ptldpytKngB8vYy2PD0fx9OnoVn0RE2CmrWCIlggUaLtszoKSVN3ECQnX/pJbWaKboXSLaSRjGLd5HFOcrM=","clientDataJSON":"eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiLXNnSXVSZ2dhbFkyRS1jLW1Mc3FJdU13Tkk5bHZhMDZYb25oR1BWVGRFRSIsIm9yaWdpbiI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MCIsImNyb3NzT3JpZ2luIjpmYWxzZX0="}}"#;
    let validate_arg = "{\"id\":\"0i99MIQfGVSKuWZVuZ4uoZnvO6ZiaYV8c6eWmHENDYM\",\"response\":{\"authenticatorData\":\"SZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2MFAAAAEQ==\",\"signature\":\"MEUCIQDqgcBDRgHx9trhh6dqdYLl5+/QHplYdeBAYs7tkUGBdAIgD3Pxq0zQ822riNF+vBfbD+DFwHNGLgL7xwE+XFeyJKs=\",\"userHandle\":null,\"clientDataJSON\":\"eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hhbGxlbmdlIjoiVUd4aFkyVm9iMnhrWlhJIiwib3JpZ2luIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwIiwiY3Jvc3NPcmlnaW4iOmZhbHNlLCJvdGhlcl9rZXlzX2Nhbl9iZV9hZGRlZF9oZXJlIjoiZG8gbm90IGNvbXBhcmUgY2xpZW50RGF0YUpTT04gYWdhaW5zdCBhIHRlbXBsYXRlLiBTZWUgaHR0cHM6Ly9nb28uZ2wveWFiUGV4In0=\"}}";

    let component = receipt1.new_component_addresses[0];
    let transaction2 = TransactionBuilder::new()
        .call_method(component, "validate", args![validate_arg])
        .build(executor.get_nonce([pk]))
        .sign([]);
    //        .sign([&sk]);
    let receipt2 = executor.validate_and_execute(&transaction2).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.result.is_ok());
}

*/

#[cfg(feature = "testing")]
mod testing {
    use super::*;
#[test]
fn test_ledger_register() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, _account) = executor.new_account();
    let package = executor.publish_package(include_package!("testing")).unwrap();

    // Test the `instantiate_hello` function.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "Tester", "test_register", args![])
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());
}

#[test]
fn test_ledger_validation() {
    // Set up environment.
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, false);
    let (pk, sk, _account) = executor.new_account();
    let package = executor.publish_package(include_package!("testing")).unwrap();

    // Test the `instantiate_hello` function.
    let transaction1 = TransactionBuilder::new()
        .call_function(package, "Tester", "test_validation", args![])
        .build(executor.get_nonce([pk]))
        .sign([&sk]);
    let receipt1 = executor.validate_and_execute(&transaction1).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.result.is_ok());
}
}