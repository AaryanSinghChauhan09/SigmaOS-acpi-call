# SigmaOS-acpi-call Component Wiki

## Overview
SigmaOS-acpi-call provides ACPI call primitives for the SigmaOS operating system. It demonstrates the ACPI (Advanced Configuration and Power Interface) call mechanism for hardware control and power management, following the principles of minimal kernel design and deterministic initialization.

## Principles
- Single Responsibility
- ACPI Call Abstraction
- Fail-Fast on ACPI Call Errors
- Deterministic ACPI Call Initialization
- Clear Abstraction Over ACPI Call APIs
- Composition over Inheritance for ACPI implementations

## Architecture
SigmaOS-acpi-call follows an ACPI call pattern:
- **Call Core**: Provides ACPI call execution and result parsing primitives
- **Hardware Control**: Abstracts hardware-specific ACPI calls for power management and device control
- **Error Handling**: Fail-fast on unsupported ACPI calls or hardware errors
- **Call Stack Management**: Manages ACPI call stack and result buffering

## API
The component provides the following public APIs:
- `acpi::call::execute(index, arguments)` - Execute an ACPI call by index
- `acpi::call::parse_result(result)` - Parse ACPI call execution result
- `acpi::call::supported(index)` - Check if an ACPI call is supported
- `acpi::call::get_status()` - Get ACPI call execution status
- Error types for unsupported calls or execution failures

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific ACPI call configuration
2. **Act**: Execute the code under test (ACPI call execution, result parsing)
3. **Assert**: Verify the expected ACPI call results

Example:
```rust
#[test]
fn test_acpi_call_execute_aaa() {
    // Arrange
    let call_index = 0x01;  // Example: Get ACPI tables call
    let arguments: Vec<u8> = vec![];
    
    // Act
    let result = acpi::call::execute(call_index, arguments);
    
    // Assert
    assert!(result.is_ok());
    let parsed = acpi::call::parse_result(result.unwrap());
    assert!(parsed.is_success());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- ACPI call initialization is best-effort; gracefully degrades on unsupported calls or hardware
- Comprehensive error types for all ACPI call failure scenarios
- Unit tests provided for ACPI call execution and result parsing
- Integration tests for full ACPI call with real firmware

## TODO
- Add support for additional ACPI call indices and arguments
- Expand hardware control capabilities (power management, device control)
- Add call result validation and error recovery
- Create integration tests with real system firmware
- Implement ACPI call logging and monitoring