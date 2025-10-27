use crate::schema;
use crate::schema::checkout_validations_generate_run::input::checkout::lines::Merchandise;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn checkout_validations_generate_run(
    input: schema::checkout_validations_generate_run::Input,
) -> Result<schema::CheckoutValidationsGenerateRunResult> {
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
        errors.push(schema::ValidationError {
            message: "A free gift was removed from your cart because your account is not eligible at this time.".to_owned(),
            target: "$.checkout".to_owned(),
        });
    }

    let operation = schema::ValidationAddOperation { errors };
    operations.push(schema::Operation::ValidationAdd(operation));

    Ok(schema::CheckoutValidationsGenerateRunResult { operations })
}