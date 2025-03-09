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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cart_items -> carts (cart_id));
diesel::joinable!(cart_items -> products (product_id));
diesel::joinable!(carts -> users (user_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart_items,
    carts,
    categories,
    order_items,
    orders,
    products,
    users,
);
