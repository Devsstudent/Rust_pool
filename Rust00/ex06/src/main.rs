
fn fibs(n: u32) -> u32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

fn is_prime(n: u32) -> bool {
    n >= 2 && !(2..n).any(|d| n % d == 0)
}

mod tests {
use crate::is_prime;
use crate::fibs;
    #[test]
    fn test_twelve_is_not_prime() {
        assert!(!is_prime(12), "twelve is not prime");
    }
    #[test]
    fn test_eleven_is_prime() {
        assert!(is_prime(11), "eleven is prime !");
    }
    #[test]
    fn test_three_is_prime() {
        assert!(is_prime(3), "three is prime !");
    }
    
    #[test]
    fn test_zero_not_prime() {
        assert!(!is_prime(0), "zero is not prime");
    }


    #[test]
    fn test_fibs() {
        assert!(fibs(32) == 2178309, "fibs funtion is working");
    }

    #[test]
    fn test_fibs_0() {
        assert!(fibs(0) == 0, "fibs test n = 0")
    }

    #[test]
    fn test_fibs_2(){
        assert!(fibs(2) == 1, "fibs test n = 2");
    }

}
