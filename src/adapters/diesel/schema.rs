diesel::table! {
    users (id) {
        id -> BigInt,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> BigInt,
        name -> Text,
        description -> Text,
        price -> Double,
        stock -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> BigInt,
        user_id -> BigInt,
        total_amount -> Double,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    order_items (id) {
        id -> BigInt,
        order_id -> BigInt,
        product_id -> BigInt,
        quantity -> Integer,
        price_at_time_of_order -> Double,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> BigInt,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    carts (id) {
        id -> BigInt,
        user_id -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cart_items (id) {
        id -> BigInt,
        cart_id -> BigInt,
        product_id -> BigInt,
        quantity -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
