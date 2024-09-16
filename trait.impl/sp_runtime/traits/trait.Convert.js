(function() {var implementors = {
"moonbase_runtime":[["impl&lt;AssetXConverter&gt; Convert&lt;<a class=\"enum\" href=\"moonbase_runtime/xcm_config/enum.CurrencyId.html\" title=\"enum moonbase_runtime::xcm_config::CurrencyId\">CurrencyId</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Location&gt;&gt; for <a class=\"struct\" href=\"moonbase_runtime/xcm_config/struct.CurrencyIdToLocation.html\" title=\"struct moonbase_runtime::xcm_config::CurrencyIdToLocation\">CurrencyIdToLocation</a>&lt;AssetXConverter&gt;<div class=\"where\">where\n    AssetXConverter: MaybeEquivalence&lt;Location, <a class=\"type\" href=\"moonbase_runtime/type.AssetId.html\" title=\"type moonbase_runtime::AssetId\">AssetId</a>&gt;,</div>"],["impl Convert&lt;&lt;&lt;<a class=\"struct\" href=\"account/struct.EthereumSignature.html\" title=\"struct account::EthereumSignature\">EthereumSignature</a> as Verify&gt;::Signer as IdentifyAccount&gt;::AccountId, H160&gt; for <a class=\"struct\" href=\"moonbase_runtime/xcm_config/struct.AccountIdToH160.html\" title=\"struct moonbase_runtime::xcm_config::AccountIdToH160\">AccountIdToH160</a>"]],
"moonbeam_runtime":[["impl Convert&lt;&lt;&lt;<a class=\"struct\" href=\"account/struct.EthereumSignature.html\" title=\"struct account::EthereumSignature\">EthereumSignature</a> as Verify&gt;::Signer as IdentifyAccount&gt;::AccountId, H160&gt; for <a class=\"struct\" href=\"moonbeam_runtime/xcm_config/struct.AccountIdToH160.html\" title=\"struct moonbeam_runtime::xcm_config::AccountIdToH160\">AccountIdToH160</a>"],["impl&lt;AssetXConverter&gt; Convert&lt;<a class=\"enum\" href=\"moonbeam_runtime/xcm_config/enum.CurrencyId.html\" title=\"enum moonbeam_runtime::xcm_config::CurrencyId\">CurrencyId</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Location&gt;&gt; for <a class=\"struct\" href=\"moonbeam_runtime/xcm_config/struct.CurrencyIdToLocation.html\" title=\"struct moonbeam_runtime::xcm_config::CurrencyIdToLocation\">CurrencyIdToLocation</a>&lt;AssetXConverter&gt;<div class=\"where\">where\n    AssetXConverter: MaybeEquivalence&lt;Location, <a class=\"type\" href=\"moonbeam_runtime/type.AssetId.html\" title=\"type moonbeam_runtime::AssetId\">AssetId</a>&gt;,</div>"]],
"moonriver_runtime":[["impl Convert&lt;&lt;&lt;<a class=\"struct\" href=\"account/struct.EthereumSignature.html\" title=\"struct account::EthereumSignature\">EthereumSignature</a> as Verify&gt;::Signer as IdentifyAccount&gt;::AccountId, H160&gt; for <a class=\"struct\" href=\"moonriver_runtime/xcm_config/struct.AccountIdToH160.html\" title=\"struct moonriver_runtime::xcm_config::AccountIdToH160\">AccountIdToH160</a>"],["impl&lt;AssetXConverter&gt; Convert&lt;<a class=\"enum\" href=\"moonriver_runtime/xcm_config/enum.CurrencyId.html\" title=\"enum moonriver_runtime::xcm_config::CurrencyId\">CurrencyId</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Location&gt;&gt; for <a class=\"struct\" href=\"moonriver_runtime/xcm_config/struct.CurrencyIdToLocation.html\" title=\"struct moonriver_runtime::xcm_config::CurrencyIdToLocation\">CurrencyIdToLocation</a>&lt;AssetXConverter&gt;<div class=\"where\">where\n    AssetXConverter: MaybeEquivalence&lt;Location, <a class=\"type\" href=\"moonriver_runtime/type.AssetId.html\" title=\"type moonriver_runtime::AssetId\">AssetId</a>&gt;,</div>"]],
"xcm_primitives":[["impl&lt;AccountId&gt; Convert&lt;AccountId, Location&gt; for <a class=\"struct\" href=\"xcm_primitives/struct.AccountIdToLocation.html\" title=\"struct xcm_primitives::AccountIdToLocation\">AccountIdToLocation</a>&lt;AccountId&gt;<div class=\"where\">where\n    AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.array.html\">20</a>]&gt;,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()