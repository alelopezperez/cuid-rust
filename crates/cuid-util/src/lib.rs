//! Common utility functions for CUID generation

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

// Construcing Base36 Values
// =========================

/// Converts any number representable as a u128 into a base36 String.
///
/// Benchmarking has shown this function to be faster than anything I've been
/// able to find in a library.
pub fn to_base_36<N: Into<u128>>(number: N) -> String {
    const RADIX: u32 = 36;
    let mut number = number.into();

    // If the number is less than the radix, it can be represented by a single
    // char. Just push that char and return.
    if number < RADIX as u128 {
        return char::from_digit(number as u32, RADIX)
            .expect("35 and under is always valid")
            .to_string();
    }

    // Allocate a string with the appropriate length for the result.
    //
    // Number of digits from n in base10 to base36 is log36(n) + 1.
    //
    // u128::MAX.log(36).trunc() is ~24, so allocating for 25 chars should always
    // be enough to avoid reallocation.
    let mut buffer = String::with_capacity(25);

    while number > 0 {
        buffer.push(
            char::from_digit((number % RADIX as u128) as u32, RADIX)
                .expect("Modulo radix always yields a valid number"),
        );
        number /= RADIX as u128;
    }

    // SAFETY: .as_mut_vec() is unsafe because it allows inserting bytes that
    // are not valid UTF-8. We are not inserting any bytes, just reversing the
    // string, so this is safe.
    unsafe {
        buffer.as_mut_vec().reverse();
    }

    buffer
}

/// Trait for types that can be converted to base 36.
pub trait ToBase36 {
    fn to_base_36(self) -> String;
}

/// Blanket impl for ToBase36 for anything that can be converted to a u128.
impl<N> ToBase36 for N
where
    N: Into<u128>,
{
    fn to_base_36(self) -> String {
        to_base_36(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn doesnt_panic(n: u128) {
            to_base_36(n);
        }

        #[test]
        fn expected_output(n: u128) {
            let val = to_base_36(n);
            assert_eq!(
                &format!("{}", radix_fmt::radix_36(n)),
                &val,
            );
            assert_eq!(
                &num::bigint::BigUint::from_bytes_be(&n.to_be_bytes()).to_str_radix(36),
                &val
            )
        }
    }
}
