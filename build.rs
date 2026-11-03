fn main() {
    // fluent-dark: koyu tema (arayüzdeki özel renkler de koyu palete göre seçildi)
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into())
        // İngilizce çeviriler ikiliye gömülür (translations/en/LC_MESSAGES/gozcu.po)
        .with_bundled_translations("translations")
        // çeviri bağlamı kullanmıyoruz: .po içindeki msgid'ler birebir eşleşir
        .with_default_translation_context(slint_build::DefaultTranslationContext::None);
    slint_build::compile_with_config("ui/app.slint", config).unwrap();
}
