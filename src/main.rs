use slint::{SharedString, ModelRc, VecModel};
slint::include_modules!();
// slint::slint!(import { MyWindow } from "ui/main.slint";);

fn main() {
    static mut PRIME: Prime = Prime::new();
    let main_window = MyWindow::new().unwrap();
    let main_weak = main_window.as_weak();

    main_window.on_check_clicked(move |input|{
        let main = main_weak.upgrade().unwrap();
        let num = input.parse::<u16>().unwrap();
        unsafe{
            if PRIME.check(num) {
                main.set_list_of_primes(ModelRc::new(VecModel::from(vec![SharedString::from("True")])));
            }
            else {
                main.set_list_of_primes(ModelRc::new(VecModel::from(vec![SharedString::from("False")])));
            }
        }
    });

    let main_weak = main_window.as_weak();
    main_window.on_count_clicked(move |input|{
        let main = main_weak.upgrade().unwrap();
        let num = input.parse::<u16>().unwrap();
        unsafe{
            main.set_list_of_primes(ModelRc::new(VecModel::from(Prime::convert(PRIME.count(num)))));
        }
    });

    main_window.run().unwrap();
}

struct Prime {
    max: u16,
    is_prime: [bool; u16::MAX as usize],
    primes: Vec<u16>,
}

impl Prime {
    pub const fn new() -> Self{
        Self { max: 0, is_prime: [false; u16::MAX as usize], primes: Vec::new() }
    }

    pub fn init(&mut self, max: u16) {
        self.primes.clear();
        for i in 2..=(max as usize) {
            if !self.is_prime[i]{
                self.primes.push(i as u16);
            }

            for p in &self.primes {
                if p * (i as u16) > max {
                    break;
                }
                self.is_prime[(*p as usize) * i] = true;
                if (i as u16) % *p == 0 {
                    break;
                }
            }
        }
        self.max = max;
    }

    pub fn check(&mut self, n: u16) -> bool {
        if self.max < n {
            self.init(n);
            self.max = n;
        }

        !self.is_prime[n as usize]
    }

    pub fn count(&mut self, n: u16) -> Vec<u16> {
        if self.max < n {
            self.init(n);
            self.max = n;
        }

        self.primes.clone().into_iter().filter(|x| x <= &n).collect()
    }

    pub fn convert(input: Vec<u16>) -> Vec<SharedString> {
        let mut ans: Vec<_> = Vec::new();
        for element in input {
            ans.push(SharedString::from(element.to_string()));
        }
        ans
    }
}