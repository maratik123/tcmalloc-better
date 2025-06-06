use std::borrow::Cow;
use std::cell::OnceCell;
use std::env;

#[derive(Debug, Copy, Clone)]
enum PageSize {
    P8k,
    P32k,
    P256k,
    PSmall,
}

impl PageSize {
    fn from_env() -> Result<PageSize, Cow<'static, str>> {
        let page_size_cell = OnceCell::new();
        for (feature, page_size) in [
            ("CARGO_FEATURE_8K_PAGES", PageSize::P8k),
            ("CARGO_FEATURE_32K_PAGES", PageSize::P32k),
            ("CARGO_FEATURE_256K_PAGES", PageSize::P256k),
            ("CARGO_FEATURE_SMALL_BUT_SLOW", PageSize::PSmall),
        ] {
            if env::var_os(feature).is_some() {
                page_size_cell
                    .set(page_size)
                    .map_err(|err| format!("Can not set up more than one page size, already defined = {page_size_cell:?}, new one = {err:?}"))?;
            }
        }
        Ok(page_size_cell
            .into_inner()
            .ok_or("One page size should be defined")?)
    }

    fn to_define(self) -> &'static str {
        match self {
            PageSize::P8k => "TCMALLOC_INTERNAL_8K_PAGES",
            PageSize::P32k => "TCMALLOC_INTERNAL_32K_PAGES",
            PageSize::P256k => "TCMALLOC_INTERNAL_256K_PAGES",
            PageSize::PSmall => "TCMALLOC_INTERNAL_SMALL_BUT_SLOW",
        }
    }
}

fn main() {
    let mut cc = cc::Build::new();
    cc.files([
        "c_src/abseil-cpp/absl/base/internal/cycleclock.cc",
        "c_src/abseil-cpp/absl/base/internal/low_level_alloc.cc",
        "c_src/abseil-cpp/absl/base/internal/raw_logging.cc",
        "c_src/abseil-cpp/absl/base/internal/spinlock.cc",
        "c_src/abseil-cpp/absl/base/internal/spinlock_wait.cc",
        "c_src/abseil-cpp/absl/base/internal/strerror.cc",
        "c_src/abseil-cpp/absl/base/internal/sysinfo.cc",
        "c_src/abseil-cpp/absl/base/internal/thread_identity.cc",
        "c_src/abseil-cpp/absl/base/internal/throw_delegate.cc",
        "c_src/abseil-cpp/absl/base/internal/unscaledcycleclock.cc",
        "c_src/abseil-cpp/absl/base/log_severity.cc",
        "c_src/abseil-cpp/absl/container/internal/hashtablez_sampler.cc",
        "c_src/abseil-cpp/absl/container/internal/hashtablez_sampler_force_weak_definition.cc",
        "c_src/abseil-cpp/absl/container/internal/raw_hash_set.cc",
        "c_src/abseil-cpp/absl/crc/crc32c.cc",
        "c_src/abseil-cpp/absl/crc/internal/cpu_detect.cc",
        "c_src/abseil-cpp/absl/crc/internal/crc.cc",
        "c_src/abseil-cpp/absl/crc/internal/crc_cord_state.cc",
        "c_src/abseil-cpp/absl/crc/internal/crc_memcpy_fallback.cc",
        "c_src/abseil-cpp/absl/crc/internal/crc_memcpy_x86_arm_combined.cc",
        "c_src/abseil-cpp/absl/crc/internal/crc_non_temporal_memcpy.cc",
        "c_src/abseil-cpp/absl/crc/internal/crc_x86_arm_combined.cc",
        "c_src/abseil-cpp/absl/debugging/internal/address_is_readable.cc",
        "c_src/abseil-cpp/absl/debugging/internal/decode_rust_punycode.cc",
        "c_src/abseil-cpp/absl/debugging/internal/demangle.cc",
        "c_src/abseil-cpp/absl/debugging/internal/demangle_rust.cc",
        "c_src/abseil-cpp/absl/debugging/internal/elf_mem_image.cc",
        "c_src/abseil-cpp/absl/debugging/internal/utf8_for_code_point.cc",
        "c_src/abseil-cpp/absl/debugging/internal/vdso_support.cc",
        "c_src/abseil-cpp/absl/debugging/stacktrace.cc",
        "c_src/abseil-cpp/absl/debugging/symbolize.cc",
        "c_src/abseil-cpp/absl/hash/internal/city.cc",
        "c_src/abseil-cpp/absl/hash/internal/hash.cc",
        "c_src/abseil-cpp/absl/hash/internal/low_level_hash.cc",
        "c_src/abseil-cpp/absl/numeric/int128.cc",
        "c_src/abseil-cpp/absl/profiling/internal/exponential_biased.cc",
        "c_src/abseil-cpp/absl/status/internal/status_internal.cc",
        "c_src/abseil-cpp/absl/status/status.cc",
        "c_src/abseil-cpp/absl/status/status_payload_printer.cc",
        "c_src/abseil-cpp/absl/status/statusor.cc",
        "c_src/abseil-cpp/absl/strings/ascii.cc",
        "c_src/abseil-cpp/absl/strings/charconv.cc",
        "c_src/abseil-cpp/absl/strings/cord.cc",
        "c_src/abseil-cpp/absl/strings/cord_analysis.cc",
        "c_src/abseil-cpp/absl/strings/cord_buffer.cc",
        "c_src/abseil-cpp/absl/strings/escaping.cc",
        "c_src/abseil-cpp/absl/strings/internal/charconv_bigint.cc",
        "c_src/abseil-cpp/absl/strings/internal/charconv_parse.cc",
        "c_src/abseil-cpp/absl/strings/internal/cord_internal.cc",
        "c_src/abseil-cpp/absl/strings/internal/cord_rep_btree.cc",
        "c_src/abseil-cpp/absl/strings/internal/cord_rep_btree_navigator.cc",
        "c_src/abseil-cpp/absl/strings/internal/cord_rep_btree_reader.cc",
        "c_src/abseil-cpp/absl/strings/internal/cord_rep_consume.cc",
        "c_src/abseil-cpp/absl/strings/internal/cord_rep_crc.cc",
        "c_src/abseil-cpp/absl/strings/internal/cordz_functions.cc",
        "c_src/abseil-cpp/absl/strings/internal/cordz_handle.cc",
        "c_src/abseil-cpp/absl/strings/internal/cordz_info.cc",
        "c_src/abseil-cpp/absl/strings/internal/damerau_levenshtein_distance.cc",
        "c_src/abseil-cpp/absl/strings/internal/escaping.cc",
        "c_src/abseil-cpp/absl/strings/internal/memutil.cc",
        "c_src/abseil-cpp/absl/strings/internal/ostringstream.cc",
        "c_src/abseil-cpp/absl/strings/internal/str_format/arg.cc",
        "c_src/abseil-cpp/absl/strings/internal/str_format/bind.cc",
        "c_src/abseil-cpp/absl/strings/internal/str_format/extension.cc",
        "c_src/abseil-cpp/absl/strings/internal/str_format/float_conversion.cc",
        "c_src/abseil-cpp/absl/strings/internal/str_format/output.cc",
        "c_src/abseil-cpp/absl/strings/internal/str_format/parser.cc",
        "c_src/abseil-cpp/absl/strings/internal/stringify_sink.cc",
        "c_src/abseil-cpp/absl/strings/internal/utf8.cc",
        "c_src/abseil-cpp/absl/strings/match.cc",
        "c_src/abseil-cpp/absl/strings/numbers.cc",
        "c_src/abseil-cpp/absl/strings/str_cat.cc",
        "c_src/abseil-cpp/absl/strings/str_replace.cc",
        "c_src/abseil-cpp/absl/strings/str_split.cc",
        "c_src/abseil-cpp/absl/strings/string_view.cc",
        "c_src/abseil-cpp/absl/strings/substitute.cc",
        "c_src/abseil-cpp/absl/synchronization/barrier.cc",
        "c_src/abseil-cpp/absl/synchronization/blocking_counter.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/create_thread_identity.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/futex_waiter.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/graphcycles.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/kernel_timeout.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/per_thread_sem.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/pthread_waiter.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/sem_waiter.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/stdcpp_waiter.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/waiter_base.cc",
        "c_src/abseil-cpp/absl/synchronization/internal/win32_waiter.cc",
        "c_src/abseil-cpp/absl/synchronization/mutex.cc",
        "c_src/abseil-cpp/absl/synchronization/notification.cc",
        "c_src/abseil-cpp/absl/time/civil_time.cc",
        "c_src/abseil-cpp/absl/time/clock.cc",
        "c_src/abseil-cpp/absl/time/duration.cc",
        "c_src/abseil-cpp/absl/time/format.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/civil_time_detail.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_fixed.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_format.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_if.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_impl.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_info.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_libc.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_lookup.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/time_zone_posix.cc",
        "c_src/abseil-cpp/absl/time/internal/cctz/src/zone_info_source.cc",
        "c_src/abseil-cpp/absl/time/time.cc",
        "c_src/abseil-cpp/absl/types/bad_optional_access.cc",
        "c_src/abseil-cpp/absl/types/bad_variant_access.cc",
        "c_src/tcmalloc/tcmalloc/allocation_sample.cc",
        "c_src/tcmalloc/tcmalloc/allocation_sampling.cc",
        "c_src/tcmalloc/tcmalloc/arena.cc",
        "c_src/tcmalloc/tcmalloc/background.cc",
        "c_src/tcmalloc/tcmalloc/central_freelist.cc",
        "c_src/tcmalloc/tcmalloc/common.cc",
        "c_src/tcmalloc/tcmalloc/cpu_cache.cc",
        "c_src/tcmalloc/tcmalloc/deallocation_profiler.cc",
        "c_src/tcmalloc/tcmalloc/error_reporting.cc",
        "c_src/tcmalloc/tcmalloc/experiment.cc",
        "c_src/tcmalloc/tcmalloc/experimental_pow2_size_class.cc",
        "c_src/tcmalloc/tcmalloc/global_stats.cc",
        "c_src/tcmalloc/tcmalloc/guarded_page_allocator.cc",
        "c_src/tcmalloc/tcmalloc/huge_address_map.cc",
        "c_src/tcmalloc/tcmalloc/huge_allocator.cc",
        "c_src/tcmalloc/tcmalloc/huge_cache.cc",
        "c_src/tcmalloc/tcmalloc/huge_page_aware_allocator.cc",
        "c_src/tcmalloc/tcmalloc/internal/allocation_guard.cc",
        "c_src/tcmalloc/tcmalloc/internal/cache_topology.cc",
        "c_src/tcmalloc/tcmalloc/internal/environment.cc",
        "c_src/tcmalloc/tcmalloc/internal/hook_list.cc",
        "c_src/tcmalloc/tcmalloc/internal/logging.cc",
        "c_src/tcmalloc/tcmalloc/internal/memory_stats.cc",
        "c_src/tcmalloc/tcmalloc/internal/memory_tag.cc",
        "c_src/tcmalloc/tcmalloc/internal/mincore.cc",
        "c_src/tcmalloc/tcmalloc/internal/numa.cc",
        "c_src/tcmalloc/tcmalloc/internal/page_size.cc",
        "c_src/tcmalloc/tcmalloc/internal/pageflags.cc",
        "c_src/tcmalloc/tcmalloc/internal/percpu.cc",
        "c_src/tcmalloc/tcmalloc/internal/percpu_rseq_asm.S",
        "c_src/tcmalloc/tcmalloc/internal/percpu_rseq_unsupported.cc",
        "c_src/tcmalloc/tcmalloc/internal/residency.cc",
        "c_src/tcmalloc/tcmalloc/internal/sysinfo.cc",
        "c_src/tcmalloc/tcmalloc/internal/util.cc",
        "c_src/tcmalloc/tcmalloc/legacy_size_classes.cc",
        "c_src/tcmalloc/tcmalloc/malloc_extension.cc",
        "c_src/tcmalloc/tcmalloc/malloc_hook.cc",
        "c_src/tcmalloc/tcmalloc/malloc_tracing_extension.cc",
        "c_src/tcmalloc/tcmalloc/page_allocator.cc",
        "c_src/tcmalloc/tcmalloc/page_allocator_interface.cc",
        "c_src/tcmalloc/tcmalloc/pagemap.cc",
        "c_src/tcmalloc/tcmalloc/parameters.cc",
        "c_src/tcmalloc/tcmalloc/peak_heap_tracker.cc",
        "c_src/tcmalloc/tcmalloc/sampler.cc",
        "c_src/tcmalloc/tcmalloc/segv_handler.cc",
        "c_src/tcmalloc/tcmalloc/selsan/selsan.cc",
        "c_src/tcmalloc/tcmalloc/size_classes.cc",
        "c_src/tcmalloc/tcmalloc/sizemap.cc",
        "c_src/tcmalloc/tcmalloc/span.cc",
        "c_src/tcmalloc/tcmalloc/stack_trace_table.cc",
        "c_src/tcmalloc/tcmalloc/static_vars.cc",
        "c_src/tcmalloc/tcmalloc/stats.cc",
        "c_src/tcmalloc/tcmalloc/system-alloc.cc",
        "c_src/tcmalloc/tcmalloc/thread_cache.cc",
        "c_src/tcmalloc/tcmalloc/transfer_cache.cc",
        "c_src/malloc_bridge.cc",
    ]);
    if env::var_os("CARGO_FEATURE_EXTENSION").is_some() {
        cc.file("c_src/malloc_extension_bridge.cc");
    }
    cc.includes(["c_src/abseil-cpp", "c_src/tcmalloc"]);
    cc.cpp(true);
    cc.std("c++17");
    cc.define("NOMINMAX", None);
    cc.define("TCMALLOC_INTERNAL_METHODS_ONLY", None);
    let page_size = PageSize::from_env().unwrap();
    cc.define(page_size.to_define(), None);
    if env::var_os("CARGO_FEATURE_DEPRECATED_PERTHREAD").is_some() {
        cc.define("TCMALLOC_DEPRECATED_PERTHREAD", None);
    }
    if env::var_os("CARGO_FEATURE_LEGACY_LOCKING").is_some() {
        cc.define("TCMALLOC_INTERNAL_LEGACY_LOCKING", None);
    }
    if env::var_os("CARGO_FEATURE_NUMA_AWARE").is_some() {
        cc.define("TCMALLOC_INTERNAL_NUMA_AWARE", None);
    }
    if match env::var_os("DEBUG") {
        Some(debug) => debug.is_empty() || debug == "0" || debug == "false",
        None => true,
    } {
        cc.define("NDEBUG", None);
    }
    cc.force_frame_pointer(true);
    cc.pic(true);
    cc.warnings(true);
    cc.extra_warnings(true);
    cc.flag_if_supported("-fno-canonical-system-headers");
    cc.flag_if_supported("-no-canonical-prefixes");
    cc.flag_if_supported("-fstack-protector");
    for warn in [
        "-Wcast-qual",
        "-Wconversion-null",
        "-Wformat-security",
        "-Wno-missing-declarations",
        "-Wno-array-bounds",
        "-Wno-attribute-alias",
        "-Wno-builtin-macro-redefined",
        "-Wno-deprecated-declarations",
        "-Wno-free-nonheap-object",
        "-Wno-sign-compare",
        "-Wno-stringop-overflow",
        "-Wno-uninitialized",
        "-Wno-unused-function",
        "-Wno-unused-result",
        "-Wno-unused-variable",
        "-Wno-unused-parameter",
        "-Wnon-virtual-dtor",
        "-Woverlength-strings",
        "-Wpointer-arith",
        "-Wno-undef",
        "-Wunused-but-set-parameter",
        "-Wunused-local-typedefs",
        "-Wvarargs",
        "-Wvla",
        "-Wwrite-strings",
        "-Wno-missing-field-initializers",
        "-Wno-type-limits",
    ] {
        cc.flag_if_supported(warn);
    }
    cc.compile("tcmalloc");
}
