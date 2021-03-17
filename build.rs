extern crate autocfg;

use autocfg::Channel;

fn main() {
    let mut ac = autocfg::new();

    ac.set_feature("re_rebalance_coherence");
    if ac.probe_expression(
        "{
            struct Foo;
            impl core::ops::Index<Foo> for str {
                type Output = str;
                fn index(&self, _: Foo) -> &str {
                    self
                }
            }
        }",
    ) {
        autocfg::emit("impl_index");
    }
    ac.set_feature("collections_range");
    if ac.probe_trait("core::ops::RangeBounds<usize>")
        && ac.probe_expression("core::ops::RangeBounds::start_bound(&(0..))")
        && ac.probe_expression("core::ops::RangeBounds::end_bound(&(0..))")
    {
        autocfg::emit("impl_range_bounds");
    }
    ac.set_feature("step_trait");
    ac.set_feature("step_trait_ext");
    ac.set_feature("unchecked_math");
    if ac.probe_trait("core::iter::Step")
        && ac.probe_expression("core::iter::Step::forward(1, 1)")
        && ac.probe_expression("core::iter::Step::forward_checked(1, 1)")
        && ac.probe_expression("unsafe { core::iter::Step::forward_unchecked(1, 1) }")
        && ac.probe_expression("core::iter::Step::backward_checked(1, 1)")
        && ac.probe_expression("unsafe { core::iter::Step::backward_unchecked(1, 1) }")
        && ac.probe_expression("core::iter::Step::steps_between(&1, &2)")
    {
        autocfg::emit("impl_iterator");
    }
    ac.set_feature("trusted_len");
    if ac.probe_trait("core::iter::TrustedLen") {
        autocfg::emit("impl_trusted_len");
    }
    ac.set_feature("doc_cfg");
    if ac.probe_expression(
        "{
            #[doc(cfg(foo))]
            struct Foo;
        }",
    ) {
        autocfg::emit("doc_cfg");
    }
    ac.emit_rustc_channel(Channel::Nightly);
}
