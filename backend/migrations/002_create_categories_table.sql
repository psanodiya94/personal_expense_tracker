-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    color VARCHAR(7),
    icon VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, name)
);

CREATE INDEX idx_categories_user_id ON categories(user_id);

-- Insert default categories
INSERT INTO categories (id, user_id, name, color, icon)
SELECT
    gen_random_uuid(),
    id,
    category_name,
    category_color,
    category_icon
FROM users
CROSS JOIN (
    VALUES
        ('Food & Dining', '#FF6B6B', '🍔'),
        ('Transportation', '#4ECDC4', '🚗'),
        ('Shopping', '#45B7D1', '🛍️'),
        ('Entertainment', '#96CEB4', '🎬'),
        ('Bills & Utilities', '#FFEAA7', '💡'),
        ('Healthcare', '#DFE6E9', '🏥'),
        ('Other', '#B2BEC3', '📦')
) AS default_categories(category_name, category_color, category_icon);
