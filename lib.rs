#![allow(non_camel_case_types, raw_pointer_deriving)]
extern crate libc;
use libc::size_t;

/* automatically generated by rust-bindgen */

pub type mp_limb_t = ::libc::c_ulong;
pub type mp_limb_signed_t = ::libc::c_long;
pub type mp_bitcnt_t = ::libc::c_ulong;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub _mp_alloc: ::libc::c_int,
    pub _mp_size: ::libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type __mpz_struct = Struct_Unnamed1;
pub type MP_INT = __mpz_struct;
pub type mpz_t = [__mpz_struct; 1u];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = ::libc::c_long;
pub type mp_exp_t = ::libc::c_long;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub _mp_num: __mpz_struct,
    pub _mp_den: __mpz_struct,
}
pub type __mpq_struct = Struct_Unnamed2;
pub type MP_RAT = __mpq_struct;
pub type mpq_t = [__mpq_struct; 1u];
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub _mp_prec: ::libc::c_int,
    pub _mp_size: ::libc::c_int,
    pub _mp_exp: mp_exp_t,
    pub _mp_d: *mut mp_limb_t,
}
pub type __mpf_struct = Struct_Unnamed3;
pub type mpf_t = [__mpf_struct; 1u];
pub type Enum_Unnamed4 = ::libc::c_uint;
pub const GMP_RAND_ALG_DEFAULT: ::libc::c_uint = 0;
pub const GMP_RAND_ALG_LC: ::libc::c_uint = 0;
pub type gmp_randalg_t = Enum_Unnamed4;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub _mp_seed: mpz_t,
    pub _mp_alg: gmp_randalg_t,
    pub _mp_algdata: Union_Unnamed6,
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed6 {
    pub data: [u64; 1u],
}
impl Union_Unnamed6 {
    pub fn _mp_lc(&mut self) -> *mut *mut ::libc::c_void {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub type __gmp_randstate_struct = Struct_Unnamed5;
pub type gmp_randstate_t = [__gmp_randstate_struct; 1u];
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
pub type mpf_srcptr = *const __mpf_struct;
pub type mpf_ptr = *mut __mpf_struct;
pub type mpq_srcptr = *const __mpq_struct;
pub type mpq_ptr = *mut __mpq_struct;
pub type Enum_Unnamed7 = ::libc::c_uint;
pub const GMP_ERROR_NONE: ::libc::c_uint = 0;
pub const GMP_ERROR_UNSUPPORTED_ARGUMENT: ::libc::c_uint = 1;
pub const GMP_ERROR_DIVISION_BY_ZERO: ::libc::c_uint = 2;
pub const GMP_ERROR_SQRT_OF_NEGATIVE: ::libc::c_uint = 4;
pub const GMP_ERROR_INVALID_ARGUMENT: ::libc::c_uint = 8;
extern "C" {
    pub static __gmp_bits_per_limb: ::libc::c_int;
    pub static mut __gmp_errno: ::libc::c_int;
    pub static __gmp_version: *const ::libc::c_char;
}
extern "C" {
    pub fn __gmp_set_memory_functions(arg1:
                                          ::std::option::Option<extern "C" fn
                                                                    (arg1:
                                                                         size_t)
                                                                    ->
                                                                        *mut ::libc::c_void>,
                                      arg2:
                                          ::std::option::Option<extern "C" fn
                                                                    (arg1:
                                                                         *mut ::libc::c_void,
                                                                     arg2:
                                                                         size_t,
                                                                     arg3:
                                                                         size_t)
                                                                    ->
                                                                        *mut ::libc::c_void>,
                                      arg3:
                                          ::std::option::Option<extern "C" fn
                                                                    (arg1:
                                                                         *mut ::libc::c_void,
                                                                     arg2:
                                                                         size_t)>);
    pub fn __gmp_get_memory_functions(arg1:
                                          *mut ::std::option::Option<extern "C" fn
                                                                         (arg1:
                                                                              size_t)
                                                                         ->
                                                                             *mut ::libc::c_void>,
                                      arg2:
                                          *mut ::std::option::Option<extern "C" fn
                                                                         (arg1:
                                                                              *mut ::libc::c_void,
                                                                          arg2:
                                                                              size_t,
                                                                          arg3:
                                                                              size_t)
                                                                         ->
                                                                             *mut ::libc::c_void>,
                                      arg3:
                                          *mut ::std::option::Option<extern "C" fn
                                                                         (arg1:
                                                                              *mut ::libc::c_void,
                                                                          arg2:
                                                                              size_t)>);
    pub fn __gmp_randinit(arg1: gmp_randstate_t, arg2: gmp_randalg_t, ...);
    pub fn __gmp_randinit_default(arg1: gmp_randstate_t);
    pub fn __gmp_randinit_lc_2exp(arg1: gmp_randstate_t, arg2: mpz_srcptr,
                                  arg3: ::libc::c_ulong, arg4: mp_bitcnt_t);
    pub fn __gmp_randinit_lc_2exp_size(arg1: gmp_randstate_t,
                                       arg2: mp_bitcnt_t) -> ::libc::c_int;
    pub fn __gmp_randinit_mt(arg1: gmp_randstate_t);
    pub fn __gmp_randinit_set(arg1: gmp_randstate_t,
                              arg2: *const __gmp_randstate_struct);
    pub fn __gmp_randseed(arg1: gmp_randstate_t, arg2: mpz_srcptr);
    pub fn __gmp_randseed_ui(arg1: gmp_randstate_t, arg2: ::libc::c_ulong);
    pub fn __gmp_randclear(arg1: gmp_randstate_t);
    pub fn __gmp_urandomb_ui(arg1: gmp_randstate_t, arg2: ::libc::c_ulong)
     -> ::libc::c_ulong;
    pub fn __gmp_urandomm_ui(arg1: gmp_randstate_t, arg2: ::libc::c_ulong)
     -> ::libc::c_ulong;
    pub fn __gmp_asprintf(arg1: *mut *mut ::libc::c_char,
                          arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn __gmp_printf(arg1: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn __gmp_snprintf(arg1: *mut ::libc::c_char, arg2: size_t,
                          arg3: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn __gmp_sprintf(arg1: *mut ::libc::c_char,
                         arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn __gmp_scanf(arg1: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn __gmp_sscanf(arg1: *const ::libc::c_char,
                        arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn __gmpz_realloc(arg1: mpz_ptr, arg2: mp_size_t)
     -> *mut ::libc::c_void;
    pub fn __gmpz_abs(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_add(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_add_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpz_addmul(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_addmul_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong);
    pub fn __gmpz_and(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_array_init(arg1: mpz_ptr, arg2: mp_size_t, arg3: mp_size_t);
    pub fn __gmpz_bin_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpz_bin_uiui(arg1: mpz_ptr, arg2: ::libc::c_ulong,
                           arg3: ::libc::c_ulong);
    pub fn __gmpz_cdiv_q(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_cdiv_q_2exp(arg1: mpz_ptr, arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_cdiv_q_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_cdiv_qr(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr,
                          arg4: mpz_srcptr);
    pub fn __gmpz_cdiv_qr_ui(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr,
                             arg4: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_cdiv_r(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_cdiv_r_2exp(arg1: mpz_ptr, arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_cdiv_r_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_cdiv_ui(arg1: mpz_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_ulong;
    pub fn __gmpz_clear(arg1: mpz_ptr);
    pub fn __gmpz_clears(arg1: mpz_ptr, ...);
    pub fn __gmpz_clrbit(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_cmp(arg1: mpz_srcptr, arg2: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_cmp_d(arg1: mpz_srcptr, arg2: ::libc::c_double)
     -> ::libc::c_int;
    pub fn __gmpz_cmp_si(arg1: mpz_srcptr, arg2: ::libc::c_long)
     -> ::libc::c_int;
    pub fn __gmpz_cmp_ui(arg1: mpz_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn __gmpz_cmpabs(arg1: mpz_srcptr, arg2: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_cmpabs_d(arg1: mpz_srcptr, arg2: ::libc::c_double)
     -> ::libc::c_int;
    pub fn __gmpz_cmpabs_ui(arg1: mpz_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn __gmpz_com(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_combit(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_congruent_p(arg1: mpz_srcptr, arg2: mpz_srcptr,
                              arg3: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_congruent_2exp_p(arg1: mpz_srcptr, arg2: mpz_srcptr,
                                   arg3: mp_bitcnt_t) -> ::libc::c_int;
    pub fn __gmpz_congruent_ui_p(arg1: mpz_srcptr, arg2: ::libc::c_ulong,
                                 arg3: ::libc::c_ulong) -> ::libc::c_int;
    pub fn __gmpz_divexact(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_divexact_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                              arg3: ::libc::c_ulong);
    pub fn __gmpz_divisible_p(arg1: mpz_srcptr, arg2: mpz_srcptr)
     -> ::libc::c_int;
    pub fn __gmpz_divisible_ui_p(arg1: mpz_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn __gmpz_divisible_2exp_p(arg1: mpz_srcptr, arg2: mp_bitcnt_t)
     -> ::libc::c_int;
    pub fn __gmpz_dump(arg1: mpz_srcptr);
    pub fn __gmpz_export(arg1: *mut ::libc::c_void, arg2: *mut size_t,
                         arg3: ::libc::c_int, arg4: size_t,
                         arg5: ::libc::c_int, arg6: size_t, arg7: mpz_srcptr)
     -> *mut ::libc::c_void;
    pub fn __gmpz_fac_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpz_2fac_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpz_mfac_uiui(arg1: mpz_ptr, arg2: ::libc::c_ulong,
                            arg3: ::libc::c_ulong);
    pub fn __gmpz_primorial_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpz_fdiv_q(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_fdiv_q_2exp(arg1: mpz_ptr, arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_fdiv_q_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_fdiv_qr(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr,
                          arg4: mpz_srcptr);
    pub fn __gmpz_fdiv_qr_ui(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr,
                             arg4: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_fdiv_r(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_fdiv_r_2exp(arg1: mpz_ptr, arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_fdiv_r_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_fdiv_ui(arg1: mpz_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_ulong;
    pub fn __gmpz_fib_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpz_fib2_ui(arg1: mpz_ptr, arg2: mpz_ptr,
                          arg3: ::libc::c_ulong);
    pub fn __gmpz_fits_sint_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_fits_slong_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_fits_sshort_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_fits_uint_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_fits_ulong_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_fits_ushort_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_gcd(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_gcd_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_gcdext(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_ptr,
                         arg4: mpz_srcptr, arg5: mpz_srcptr);
    pub fn __gmpz_get_d(arg1: mpz_srcptr) -> ::libc::c_double;
    pub fn __gmpz_get_d_2exp(arg1: *mut ::libc::c_long, arg2: mpz_srcptr)
     -> ::libc::c_double;
    pub fn __gmpz_get_si(arg1: mpz_srcptr) -> ::libc::c_long;
    pub fn __gmpz_get_str(arg1: *mut ::libc::c_char, arg2: ::libc::c_int,
                          arg3: mpz_srcptr) -> *mut ::libc::c_char;
    pub fn __gmpz_get_ui(arg1: mpz_srcptr) -> ::libc::c_ulong;
    pub fn __gmpz_getlimbn(arg1: mpz_srcptr, arg2: mp_size_t) -> mp_limb_t;
    pub fn __gmpz_hamdist(arg1: mpz_srcptr, arg2: mpz_srcptr) -> mp_bitcnt_t;
    pub fn __gmpz_import(arg1: mpz_ptr, arg2: size_t, arg3: ::libc::c_int,
                         arg4: size_t, arg5: ::libc::c_int, arg6: size_t,
                         arg7: *const ::libc::c_void);
    pub fn __gmpz_init(arg1: mpz_ptr);
    pub fn __gmpz_init2(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_inits(arg1: mpz_ptr, ...);
    pub fn __gmpz_init_set(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_init_set_d(arg1: mpz_ptr, arg2: ::libc::c_double);
    pub fn __gmpz_init_set_si(arg1: mpz_ptr, arg2: ::libc::c_long);
    pub fn __gmpz_init_set_str(arg1: mpz_ptr, arg2: *const ::libc::c_char,
                               arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn __gmpz_init_set_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpz_invert(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr)
     -> ::libc::c_int;
    pub fn __gmpz_ior(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_jacobi(arg1: mpz_srcptr, arg2: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_kronecker_si(arg1: mpz_srcptr, arg2: ::libc::c_long)
     -> ::libc::c_int;
    pub fn __gmpz_kronecker_ui(arg1: mpz_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn __gmpz_si_kronecker(arg1: ::libc::c_long, arg2: mpz_srcptr)
     -> ::libc::c_int;
    pub fn __gmpz_ui_kronecker(arg1: ::libc::c_ulong, arg2: mpz_srcptr)
     -> ::libc::c_int;
    pub fn __gmpz_lcm(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_lcm_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpz_lucnum_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpz_lucnum2_ui(arg1: mpz_ptr, arg2: mpz_ptr,
                             arg3: ::libc::c_ulong);
    pub fn __gmpz_millerrabin(arg1: mpz_srcptr, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn __gmpz_mod(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_mul(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_mul_2exp(arg1: mpz_ptr, arg2: mpz_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpz_mul_si(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_long);
    pub fn __gmpz_mul_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpz_neg(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_nextprime(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_perfect_power_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_perfect_square_p(arg1: mpz_srcptr) -> ::libc::c_int;
    pub fn __gmpz_popcount(arg1: mpz_srcptr) -> mp_bitcnt_t;
    pub fn __gmpz_pow_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpz_powm(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr,
                       arg4: mpz_srcptr);
    pub fn __gmpz_powm_sec(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr,
                           arg4: mpz_srcptr);
    pub fn __gmpz_powm_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                          arg3: ::libc::c_ulong, arg4: mpz_srcptr);
    pub fn __gmpz_probab_prime_p(arg1: mpz_srcptr, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn __gmpz_random(arg1: mpz_ptr, arg2: mp_size_t);
    pub fn __gmpz_random2(arg1: mpz_ptr, arg2: mp_size_t);
    pub fn __gmpz_realloc2(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_remove(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr)
     -> mp_bitcnt_t;
    pub fn __gmpz_root(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn __gmpz_rootrem(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr,
                          arg4: ::libc::c_ulong);
    pub fn __gmpz_rrandomb(arg1: mpz_ptr, arg2: gmp_randstate_t,
                           arg3: mp_bitcnt_t);
    pub fn __gmpz_scan0(arg1: mpz_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpz_scan1(arg1: mpz_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpz_set(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_set_d(arg1: mpz_ptr, arg2: ::libc::c_double);
    pub fn __gmpz_set_f(arg1: mpz_ptr, arg2: mpf_srcptr);
    pub fn __gmpz_set_q(arg1: mpz_ptr, arg2: mpq_srcptr);
    pub fn __gmpz_set_si(arg1: mpz_ptr, arg2: ::libc::c_long);
    pub fn __gmpz_set_str(arg1: mpz_ptr, arg2: *const ::libc::c_char,
                          arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn __gmpz_set_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpz_setbit(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_size(arg1: mpz_srcptr) -> size_t;
    pub fn __gmpz_sizeinbase(arg1: mpz_srcptr, arg2: ::libc::c_int) -> size_t;
    pub fn __gmpz_sqrt(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_sqrtrem(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr);
    pub fn __gmpz_sub(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_sub_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpz_ui_sub(arg1: mpz_ptr, arg2: ::libc::c_ulong,
                         arg3: mpz_srcptr);
    pub fn __gmpz_submul(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_submul_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong);
    pub fn __gmpz_swap(arg1: mpz_ptr, arg2: mpz_ptr);
    pub fn __gmpz_tdiv_ui(arg1: mpz_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_ulong;
    pub fn __gmpz_tdiv_q(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_tdiv_q_2exp(arg1: mpz_ptr, arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_tdiv_q_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_tdiv_qr(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr,
                          arg4: mpz_srcptr);
    pub fn __gmpz_tdiv_qr_ui(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr,
                             arg4: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_tdiv_r(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_tdiv_r_2exp(arg1: mpz_ptr, arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_tdiv_r_ui(arg1: mpz_ptr, arg2: mpz_srcptr,
                            arg3: ::libc::c_ulong) -> ::libc::c_ulong;
    pub fn __gmpz_tstbit(arg1: mpz_srcptr, arg2: mp_bitcnt_t)
     -> ::libc::c_int;
    pub fn __gmpz_ui_pow_ui(arg1: mpz_ptr, arg2: ::libc::c_ulong,
                            arg3: ::libc::c_ulong);
    pub fn __gmpz_urandomb(arg1: mpz_ptr, arg2: gmp_randstate_t,
                           arg3: mp_bitcnt_t);
    pub fn __gmpz_urandomm(arg1: mpz_ptr, arg2: gmp_randstate_t,
                           arg3: mpz_srcptr);
    pub fn __gmpz_xor(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_limbs_read(arg1: mpz_srcptr) -> mp_srcptr;
    pub fn __gmpz_limbs_write(arg1: mpz_ptr, arg2: mp_size_t) -> mp_ptr;
    pub fn __gmpz_limbs_modify(arg1: mpz_ptr, arg2: mp_size_t) -> mp_ptr;
    pub fn __gmpz_limbs_finish(arg1: mpz_ptr, arg2: mp_size_t);
    pub fn __gmpz_roinit_n(arg1: mpz_ptr, arg2: mp_srcptr, arg3: mp_size_t)
     -> mpz_srcptr;
    pub fn __gmpq_abs(arg1: mpq_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_add(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_canonicalize(arg1: mpq_ptr);
    pub fn __gmpq_clear(arg1: mpq_ptr);
    pub fn __gmpq_clears(arg1: mpq_ptr, ...);
    pub fn __gmpq_cmp(arg1: mpq_srcptr, arg2: mpq_srcptr) -> ::libc::c_int;
    pub fn __gmpq_cmp_si(arg1: mpq_srcptr, arg2: ::libc::c_long,
                         arg3: ::libc::c_ulong) -> ::libc::c_int;
    pub fn __gmpq_cmp_ui(arg1: mpq_srcptr, arg2: ::libc::c_ulong,
                         arg3: ::libc::c_ulong) -> ::libc::c_int;
    pub fn __gmpq_div(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_div_2exp(arg1: mpq_ptr, arg2: mpq_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpq_equal(arg1: mpq_srcptr, arg2: mpq_srcptr) -> ::libc::c_int;
    pub fn __gmpq_get_num(arg1: mpz_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_get_den(arg1: mpz_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_get_d(arg1: mpq_srcptr) -> ::libc::c_double;
    pub fn __gmpq_get_str(arg1: *mut ::libc::c_char, arg2: ::libc::c_int,
                          arg3: mpq_srcptr) -> *mut ::libc::c_char;
    pub fn __gmpq_init(arg1: mpq_ptr);
    pub fn __gmpq_inits(arg1: mpq_ptr, ...);
    pub fn __gmpq_inv(arg1: mpq_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_mul(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_mul_2exp(arg1: mpq_ptr, arg2: mpq_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpq_neg(arg1: mpq_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_set(arg1: mpq_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_set_d(arg1: mpq_ptr, arg2: ::libc::c_double);
    pub fn __gmpq_set_den(arg1: mpq_ptr, arg2: mpz_srcptr);
    pub fn __gmpq_set_f(arg1: mpq_ptr, arg2: mpf_srcptr);
    pub fn __gmpq_set_num(arg1: mpq_ptr, arg2: mpz_srcptr);
    pub fn __gmpq_set_si(arg1: mpq_ptr, arg2: ::libc::c_long,
                         arg3: ::libc::c_ulong);
    pub fn __gmpq_set_str(arg1: mpq_ptr, arg2: *const ::libc::c_char,
                          arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn __gmpq_set_ui(arg1: mpq_ptr, arg2: ::libc::c_ulong,
                         arg3: ::libc::c_ulong);
    pub fn __gmpq_set_z(arg1: mpq_ptr, arg2: mpz_srcptr);
    pub fn __gmpq_sub(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_swap(arg1: mpq_ptr, arg2: mpq_ptr);
    pub fn __gmpf_abs(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_add(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_add_ui(arg1: mpf_ptr, arg2: mpf_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpf_ceil(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_clear(arg1: mpf_ptr);
    pub fn __gmpf_clears(arg1: mpf_ptr, ...);
    pub fn __gmpf_cmp(arg1: mpf_srcptr, arg2: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_cmp_d(arg1: mpf_srcptr, arg2: ::libc::c_double)
     -> ::libc::c_int;
    pub fn __gmpf_cmp_si(arg1: mpf_srcptr, arg2: ::libc::c_long)
     -> ::libc::c_int;
    pub fn __gmpf_cmp_ui(arg1: mpf_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn __gmpf_div(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_div_2exp(arg1: mpf_ptr, arg2: mpf_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpf_div_ui(arg1: mpf_ptr, arg2: mpf_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpf_dump(arg1: mpf_srcptr);
    pub fn __gmpf_eq(arg1: mpf_srcptr, arg2: mpf_srcptr, arg3: mp_bitcnt_t)
     -> ::libc::c_int;
    pub fn __gmpf_fits_sint_p(arg1: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_fits_slong_p(arg1: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_fits_sshort_p(arg1: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_fits_uint_p(arg1: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_fits_ulong_p(arg1: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_fits_ushort_p(arg1: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_floor(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_get_d(arg1: mpf_srcptr) -> ::libc::c_double;
    pub fn __gmpf_get_d_2exp(arg1: *mut ::libc::c_long, arg2: mpf_srcptr)
     -> ::libc::c_double;
    pub fn __gmpf_get_default_prec() -> mp_bitcnt_t;
    pub fn __gmpf_get_prec(arg1: mpf_srcptr) -> mp_bitcnt_t;
    pub fn __gmpf_get_si(arg1: mpf_srcptr) -> ::libc::c_long;
    pub fn __gmpf_get_str(arg1: *mut ::libc::c_char, arg2: *mut mp_exp_t,
                          arg3: ::libc::c_int, arg4: size_t, arg5: mpf_srcptr)
     -> *mut ::libc::c_char;
    pub fn __gmpf_get_ui(arg1: mpf_srcptr) -> ::libc::c_ulong;
    pub fn __gmpf_init(arg1: mpf_ptr);
    pub fn __gmpf_init2(arg1: mpf_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpf_inits(arg1: mpf_ptr, ...);
    pub fn __gmpf_init_set(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_init_set_d(arg1: mpf_ptr, arg2: ::libc::c_double);
    pub fn __gmpf_init_set_si(arg1: mpf_ptr, arg2: ::libc::c_long);
    pub fn __gmpf_init_set_str(arg1: mpf_ptr, arg2: *const ::libc::c_char,
                               arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn __gmpf_init_set_ui(arg1: mpf_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpf_integer_p(arg1: mpf_srcptr) -> ::libc::c_int;
    pub fn __gmpf_mul(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_mul_2exp(arg1: mpf_ptr, arg2: mpf_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpf_mul_ui(arg1: mpf_ptr, arg2: mpf_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpf_neg(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_pow_ui(arg1: mpf_ptr, arg2: mpf_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpf_random2(arg1: mpf_ptr, arg2: mp_size_t, arg3: mp_exp_t);
    pub fn __gmpf_reldiff(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_set(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_set_d(arg1: mpf_ptr, arg2: ::libc::c_double);
    pub fn __gmpf_set_default_prec(arg1: mp_bitcnt_t);
    pub fn __gmpf_set_prec(arg1: mpf_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpf_set_prec_raw(arg1: mpf_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpf_set_q(arg1: mpf_ptr, arg2: mpq_srcptr);
    pub fn __gmpf_set_si(arg1: mpf_ptr, arg2: ::libc::c_long);
    pub fn __gmpf_set_str(arg1: mpf_ptr, arg2: *const ::libc::c_char,
                          arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn __gmpf_set_ui(arg1: mpf_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpf_set_z(arg1: mpf_ptr, arg2: mpz_srcptr);
    pub fn __gmpf_size(arg1: mpf_srcptr) -> size_t;
    pub fn __gmpf_sqrt(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_sqrt_ui(arg1: mpf_ptr, arg2: ::libc::c_ulong);
    pub fn __gmpf_sub(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_sub_ui(arg1: mpf_ptr, arg2: mpf_srcptr,
                         arg3: ::libc::c_ulong);
    pub fn __gmpf_swap(arg1: mpf_ptr, arg2: mpf_ptr);
    pub fn __gmpf_trunc(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_ui_div(arg1: mpf_ptr, arg2: ::libc::c_ulong,
                         arg3: mpf_srcptr);
    pub fn __gmpf_ui_sub(arg1: mpf_ptr, arg2: ::libc::c_ulong,
                         arg3: mpf_srcptr);
    pub fn __gmpf_urandomb(arg1: mpf_t, arg2: gmp_randstate_t,
                           arg3: mp_bitcnt_t);
    pub fn __gmpn_add(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                      arg4: mp_srcptr, arg5: mp_size_t) -> mp_limb_t;
    pub fn __gmpn_add_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                        arg4: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_add_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                        arg4: mp_size_t) -> mp_limb_t;
    pub fn __gmpn_addmul_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                           arg4: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_cmp(arg1: mp_srcptr, arg2: mp_srcptr, arg3: mp_size_t)
     -> ::libc::c_int;
    pub fn __gmpn_divexact_by3c(arg1: mp_ptr, arg2: mp_srcptr,
                                arg3: mp_size_t, arg4: mp_limb_t)
     -> mp_limb_t;
    pub fn __gmpn_divrem(arg1: mp_ptr, arg2: mp_size_t, arg3: mp_ptr,
                         arg4: mp_size_t, arg5: mp_srcptr, arg6: mp_size_t)
     -> mp_limb_t;
    pub fn __gmpn_divrem_1(arg1: mp_ptr, arg2: mp_size_t, arg3: mp_srcptr,
                           arg4: mp_size_t, arg5: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_divrem_2(arg1: mp_ptr, arg2: mp_size_t, arg3: mp_ptr,
                           arg4: mp_size_t, arg5: mp_srcptr) -> mp_limb_t;
    pub fn __gmpn_div_qr_1(arg1: mp_ptr, arg2: *mut mp_limb_t,
                           arg3: mp_srcptr, arg4: mp_size_t, arg5: mp_limb_t)
     -> mp_limb_t;
    pub fn __gmpn_div_qr_2(arg1: mp_ptr, arg2: mp_ptr, arg3: mp_srcptr,
                           arg4: mp_size_t, arg5: mp_srcptr) -> mp_limb_t;
    pub fn __gmpn_gcd(arg1: mp_ptr, arg2: mp_ptr, arg3: mp_size_t,
                      arg4: mp_ptr, arg5: mp_size_t) -> mp_size_t;
    pub fn __gmpn_gcd_1(arg1: mp_srcptr, arg2: mp_size_t, arg3: mp_limb_t)
     -> mp_limb_t;
    pub fn __gmpn_gcdext_1(arg1: *mut mp_limb_signed_t,
                           arg2: *mut mp_limb_signed_t, arg3: mp_limb_t,
                           arg4: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_gcdext(arg1: mp_ptr, arg2: mp_ptr, arg3: *mut mp_size_t,
                         arg4: mp_ptr, arg5: mp_size_t, arg6: mp_ptr,
                         arg7: mp_size_t) -> mp_size_t;
    pub fn __gmpn_get_str(arg1: *mut ::libc::c_uchar, arg2: ::libc::c_int,
                          arg3: mp_ptr, arg4: mp_size_t) -> size_t;
    pub fn __gmpn_hamdist(arg1: mp_srcptr, arg2: mp_srcptr, arg3: mp_size_t)
     -> mp_bitcnt_t;
    pub fn __gmpn_lshift(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                         arg4: ::libc::c_uint) -> mp_limb_t;
    pub fn __gmpn_mod_1(arg1: mp_srcptr, arg2: mp_size_t, arg3: mp_limb_t)
     -> mp_limb_t;
    pub fn __gmpn_mul(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                      arg4: mp_srcptr, arg5: mp_size_t) -> mp_limb_t;
    pub fn __gmpn_mul_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                        arg4: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_mul_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_sqr(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_neg(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t)
     -> mp_limb_t;
    pub fn __gmpn_com(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_perfect_square_p(arg1: mp_srcptr, arg2: mp_size_t)
     -> ::libc::c_int;
    pub fn __gmpn_perfect_power_p(arg1: mp_srcptr, arg2: mp_size_t)
     -> ::libc::c_int;
    pub fn __gmpn_popcount(arg1: mp_srcptr, arg2: mp_size_t) -> mp_bitcnt_t;
    pub fn __gmpn_pow_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                        arg4: mp_limb_t, arg5: mp_ptr) -> mp_size_t;
    pub fn __gmpn_preinv_mod_1(arg1: mp_srcptr, arg2: mp_size_t,
                               arg3: mp_limb_t, arg4: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_random(arg1: mp_ptr, arg2: mp_size_t);
    pub fn __gmpn_random2(arg1: mp_ptr, arg2: mp_size_t);
    pub fn __gmpn_rshift(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                         arg4: ::libc::c_uint) -> mp_limb_t;
    pub fn __gmpn_scan0(arg1: mp_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpn_scan1(arg1: mp_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpn_set_str(arg1: mp_ptr, arg2: *const ::libc::c_uchar,
                          arg3: size_t, arg4: ::libc::c_int) -> mp_size_t;
    pub fn __gmpn_sizeinbase(arg1: mp_srcptr, arg2: mp_size_t,
                             arg3: ::libc::c_int) -> size_t;
    pub fn __gmpn_sqrtrem(arg1: mp_ptr, arg2: mp_ptr, arg3: mp_srcptr,
                          arg4: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sub(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                      arg4: mp_srcptr, arg5: mp_size_t) -> mp_limb_t;
    pub fn __gmpn_sub_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                        arg4: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_sub_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                        arg4: mp_size_t) -> mp_limb_t;
    pub fn __gmpn_submul_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                           arg4: mp_limb_t) -> mp_limb_t;
    pub fn __gmpn_tdiv_qr(arg1: mp_ptr, arg2: mp_ptr, arg3: mp_size_t,
                          arg4: mp_srcptr, arg5: mp_size_t, arg6: mp_srcptr,
                          arg7: mp_size_t);
    pub fn __gmpn_and_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_andn_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_nand_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_ior_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_iorn_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_nior_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_xor_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_xnor_n(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_copyi(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_copyd(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_zero(arg1: mp_ptr, arg2: mp_size_t);
    pub fn __gmpn_cnd_add_n(arg1: mp_limb_t, arg2: mp_ptr, arg3: mp_srcptr,
                            arg4: mp_srcptr, arg5: mp_size_t) -> mp_limb_t;
    pub fn __gmpn_cnd_sub_n(arg1: mp_limb_t, arg2: mp_ptr, arg3: mp_srcptr,
                            arg4: mp_srcptr, arg5: mp_size_t) -> mp_limb_t;
    pub fn __gmpn_sec_add_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                            arg4: mp_limb_t, arg5: mp_ptr) -> mp_limb_t;
    pub fn __gmpn_sec_add_1_itch(arg1: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_sub_1(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                            arg4: mp_limb_t, arg5: mp_ptr) -> mp_limb_t;
    pub fn __gmpn_sec_sub_1_itch(arg1: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_mul(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                          arg4: mp_srcptr, arg5: mp_size_t, arg6: mp_ptr);
    pub fn __gmpn_sec_mul_itch(arg1: mp_size_t, arg2: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_sqr(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                          arg4: mp_ptr);
    pub fn __gmpn_sec_sqr_itch(arg1: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_powm(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t,
                           arg4: mp_srcptr, arg5: mp_bitcnt_t,
                           arg6: mp_srcptr, arg7: mp_size_t, arg8: mp_ptr);
    pub fn __gmpn_sec_powm_itch(arg1: mp_size_t, arg2: mp_bitcnt_t,
                                arg3: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_tabselect(arg1: *mut mp_limb_t, arg2: *const mp_limb_t,
                                arg3: mp_size_t, arg4: mp_size_t,
                                arg5: mp_size_t);
    pub fn __gmpn_sec_div_qr(arg1: mp_ptr, arg2: mp_ptr, arg3: mp_size_t,
                             arg4: mp_srcptr, arg5: mp_size_t, arg6: mp_ptr)
     -> mp_limb_t;
    pub fn __gmpn_sec_div_qr_itch(arg1: mp_size_t, arg2: mp_size_t)
     -> mp_size_t;
    pub fn __gmpn_sec_div_r(arg1: mp_ptr, arg2: mp_size_t, arg3: mp_srcptr,
                            arg4: mp_size_t, arg5: mp_ptr);
    pub fn __gmpn_sec_div_r_itch(arg1: mp_size_t, arg2: mp_size_t)
     -> mp_size_t;
    pub fn __gmpn_sec_invert(arg1: mp_ptr, arg2: mp_ptr, arg3: mp_srcptr,
                             arg4: mp_size_t, arg5: mp_bitcnt_t, arg6: mp_ptr)
     -> ::libc::c_int;
    pub fn __gmpn_sec_invert_itch(arg1: mp_size_t) -> mp_size_t;
}
