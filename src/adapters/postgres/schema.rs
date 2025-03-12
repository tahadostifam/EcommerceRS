// @generated automatically by Diesel CLI.

diesel::table! {
    cart_items (id) {
        id -> Int8,
        cart_id -> Int8,
        product_id -> Int8,
        quantity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    carts (id) {
        id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int8,
        name -> Text,
        description -> Text,
        parent_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    order_items (id) {
        id -> Int8,
        order_id -> Int8,
        product_id -> Int8,
        quantity -> Int4,
        price_at_time_of_order -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Int8,
        user_id -> Int8,
        total_amount -> Float8,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int8,
        name -> Text,
        description -> Text,
        price -> Float8,
        stock -> Int4,
        product_image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        category_id -> Nullable<Int8>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        profile_picture -> Nullable<Text>,
        password_hash -> Text,
        email_verified -> Bool,
        user_role -> Text,
        last_login -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    variation_options (id) {
        id -> Int8,
        variation_id -> Int8,
        value -> Text,
    }
}

diesel::table! {
    variations (id) {
        id -> Int8,
        category_id -> Int8,
        name -> Text,
    }
}

diesel::joinable!(cart_items -> carts (cart_id));
diesel::joinable!(cart_items -> products (product_id));
diesel::joinable!(carts -> users (user_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(variation_options -> variations (variation_id));
diesel::joinable!(variations -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart_items,
    carts,
    categories,
    order_items,
    orders,
    products,
    users,
    variation_options,
    variations,
);
