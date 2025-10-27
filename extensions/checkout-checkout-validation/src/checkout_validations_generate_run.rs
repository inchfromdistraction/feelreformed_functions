use shopify_function::prelude::*;
use shopify_function::Result;
use crate::checkout_validations_generate_run::input::checkout::lines::Merchandise;

#[shopify_function]
fn checkout_validations_generate_run(
    input: checkout_validations_generate_run::Input,
) -> Result<checkout_validations_generate_run::Result> {
    let mut operations = Vec::new();
    let mut errors = Vec::new();

    // List of gift variant IDs
    let gift_variant_ids = [
        "gid://shopify/ProductVariant/55946981966201",
        "gid://shopify/ProductVariant/55697507516793",
        "gid://shopify/ProductVariant/55946984554873",
        "gid://shopify/ProductVariant/55946979311993",
        "gid://shopify/ProductVariant/55946874028409",
    ];

    // Check for gift variants in checkout
    let has_gift = input
        .checkout()
        .lines()
        .iter()
        .any(|line| match line.merchandise() {
            Merchandise::ProductVariant(product_variant) =>
                gift_variant_ids.iter().any(|gid| *gid == product_variant.id().as_str()),
            _ => false,
        });

    if has_gift {
        errors.push(checkout_validations_generate_run::ValidationError {
            message: "A free gift was removed from your cart because your account is not eligible at this time.".to_owned(),
            target: "$.checkout".to_owned(),
        });
    }

    let operation = checkout_validations_generate_run::ValidationAddOperation { errors };
    operations.push(checkout_validations_generate_run::Operation::ValidationAdd(operation));

    Ok(checkout_validations_generate_run::Result { operations })
}