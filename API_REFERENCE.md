# API Reference - Expense Tracker

Complete API documentation for the Personal Expense Tracker backend.

## Base URL

```
http://localhost:3000/api
```

## Table of Contents

1. [Authentication](#authentication)
2. [Users](#users)
3. [Categories](#categories)
4. [Expenses](#expenses)
5. [Summaries](#summaries)
6. [Error Responses](#error-responses)
7. [Request Examples](#request-examples)

---

## Authentication

All endpoints except registration and login require a JWT token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

### Register New User

Creates a new user account.

**Endpoint:** `POST /auth/register`

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123",
  "full_name": "John Doe"
}
```

**Validation Rules:**
- `email`: Must be a valid email format
- `password`: Minimum 8 characters
- `full_name`: At least 1 character

**Response:** `201 Created`
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "user@example.com",
    "full_name": "John Doe",
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

**Error Responses:**
- `400 Bad Request` - Validation failed or email already exists
  ```json
  {
    "error": "Email already registered"
  }
  ```

---

### Login

Authenticates a user and returns a JWT token.

**Endpoint:** `POST /auth/login`

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response:** `200 OK`
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "user@example.com",
    "full_name": "John Doe",
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

**Error Responses:**
- `401 Unauthorized` - Invalid credentials
  ```json
  {
    "error": "Invalid credentials"
  }
  ```

---

## Users

### Get Current User

Returns information about the currently authenticated user.

**Endpoint:** `GET /users/me`

**Headers:**
```
Authorization: Bearer <token>
```

**Response:** `200 OK`
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "user@example.com",
  "full_name": "John Doe",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Responses:**
- `401 Unauthorized` - Missing or invalid token
- `404 Not Found` - User not found

---

## Categories

### List Categories

Returns all categories for the authenticated user.

**Endpoint:** `GET /categories`

**Headers:**
```
Authorization: Bearer <token>
```

**Response:** `200 OK`
```json
[
  {
    "id": "cat-uuid-1",
    "user_id": "user-uuid",
    "name": "Food & Dining",
    "color": "#FF6B6B",
    "icon": "🍔",
    "created_at": "2024-01-15T10:30:00Z"
  },
  {
    "id": "cat-uuid-2",
    "user_id": "user-uuid",
    "name": "Transportation",
    "color": "#4ECDC4",
    "icon": "🚗",
    "created_at": "2024-01-15T10:30:00Z"
  }
]
```

---

### Create Category

Creates a new expense category.

**Endpoint:** `POST /categories`

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "name": "Groceries",
  "color": "#45B7D1",
  "icon": "🛒"
}
```

**Validation Rules:**
- `name`: 1-100 characters, must be unique for the user
- `color`: Optional hex color code
- `icon`: Optional emoji or icon identifier

**Response:** `201 Created`
```json
{
  "id": "new-cat-uuid",
  "user_id": "user-uuid",
  "name": "Groceries",
  "color": "#45B7D1",
  "icon": "🛒",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Responses:**
- `400 Bad Request` - Validation failed or category name already exists
  ```json
  {
    "error": "Category name already exists"
  }
  ```

---

### Get Category

Returns a specific category by ID.

**Endpoint:** `GET /categories/:id`

**Headers:**
```
Authorization: Bearer <token>
```

**URL Parameters:**
- `id` - Category UUID

**Response:** `200 OK`
```json
{
  "id": "cat-uuid",
  "user_id": "user-uuid",
  "name": "Food & Dining",
  "color": "#FF6B6B",
  "icon": "🍔",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Responses:**
- `404 Not Found` - Category not found or doesn't belong to user

---

### Update Category

Updates an existing category.

**Endpoint:** `PUT /categories/:id`

**Headers:**
```
Authorization: Bearer <token>
```

**URL Parameters:**
- `id` - Category UUID

**Request Body:** (all fields optional)
```json
{
  "name": "Grocery Shopping",
  "color": "#45B7D1",
  "icon": "🛒"
}
```

**Response:** `200 OK`
```json
{
  "id": "cat-uuid",
  "user_id": "user-uuid",
  "name": "Grocery Shopping",
  "color": "#45B7D1",
  "icon": "🛒",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Responses:**
- `400 Bad Request` - Validation failed or no fields to update
- `404 Not Found` - Category not found

---

### Delete Category

Deletes a category.

**Endpoint:** `DELETE /categories/:id`

**Headers:**
```
Authorization: Bearer <token>
```

**URL Parameters:**
- `id` - Category UUID

**Response:** `204 No Content`

**Error Responses:**
- `400 Bad Request` - Category has associated expenses
  ```json
  {
    "error": "Cannot delete category with existing expenses"
  }
  ```
- `404 Not Found` - Category not found

---

## Expenses

### List Expenses

Returns expenses for the authenticated user with optional filtering.

**Endpoint:** `GET /expenses`

**Headers:**
```
Authorization: Bearer <token>
```

**Query Parameters:** (all optional)
- `start_date` - Filter expenses from this date (ISO 8601: YYYY-MM-DD)
- `end_date` - Filter expenses up to this date (ISO 8601: YYYY-MM-DD)
- `category_id` - Filter by category UUID

**Example URLs:**
```
GET /expenses
GET /expenses?start_date=2024-01-01&end_date=2024-01-31
GET /expenses?category_id=cat-uuid
GET /expenses?start_date=2024-01-01&category_id=cat-uuid
```

**Response:** `200 OK`
```json
[
  {
    "id": "exp-uuid-1",
    "user_id": "user-uuid",
    "category_id": "cat-uuid",
    "category_name": "Food & Dining",
    "category_color": "#FF6B6B",
    "category_icon": "🍔",
    "amount": "42.50",
    "description": "Lunch at restaurant",
    "expense_date": "2024-01-15",
    "created_at": "2024-01-15T14:30:00Z",
    "updated_at": "2024-01-15T14:30:00Z"
  }
]
```

---

### Create Expense

Creates a new expense record.

**Endpoint:** `POST /expenses`

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "category_id": "cat-uuid",
  "amount": 42.50,
  "description": "Lunch at restaurant",
  "expense_date": "2024-01-15"
}
```

**Validation Rules:**
- `category_id`: Must be a valid category belonging to the user
- `amount`: Must be greater than 0
- `description`: At least 1 character
- `expense_date`: Valid date in ISO 8601 format (YYYY-MM-DD)

**Response:** `201 Created`
```json
{
  "id": "new-exp-uuid",
  "user_id": "user-uuid",
  "category_id": "cat-uuid",
  "category_name": "Food & Dining",
  "category_color": "#FF6B6B",
  "category_icon": "🍔",
  "amount": "42.50",
  "description": "Lunch at restaurant",
  "expense_date": "2024-01-15",
  "created_at": "2024-01-15T14:30:00Z",
  "updated_at": "2024-01-15T14:30:00Z"
}
```

**Error Responses:**
- `400 Bad Request` - Validation failed
- `404 Not Found` - Category not found

---

### Get Expense

Returns a specific expense by ID.

**Endpoint:** `GET /expenses/:id`

**Headers:**
```
Authorization: Bearer <token>
```

**URL Parameters:**
- `id` - Expense UUID

**Response:** `200 OK`
```json
{
  "id": "exp-uuid",
  "user_id": "user-uuid",
  "category_id": "cat-uuid",
  "category_name": "Food & Dining",
  "category_color": "#FF6B6B",
  "category_icon": "🍔",
  "amount": "42.50",
  "description": "Lunch at restaurant",
  "expense_date": "2024-01-15",
  "created_at": "2024-01-15T14:30:00Z",
  "updated_at": "2024-01-15T14:30:00Z"
}
```

**Error Responses:**
- `404 Not Found` - Expense not found

---

### Update Expense

Updates an existing expense.

**Endpoint:** `PUT /expenses/:id`

**Headers:**
```
Authorization: Bearer <token>
```

**URL Parameters:**
- `id` - Expense UUID

**Request Body:** (all fields optional)
```json
{
  "category_id": "new-cat-uuid",
  "amount": 45.00,
  "description": "Updated description",
  "expense_date": "2024-01-16"
}
```

**Response:** `200 OK`
```json
{
  "id": "exp-uuid",
  "user_id": "user-uuid",
  "category_id": "new-cat-uuid",
  "category_name": "Updated Category",
  "category_color": "#45B7D1",
  "category_icon": "🛒",
  "amount": "45.00",
  "description": "Updated description",
  "expense_date": "2024-01-16",
  "created_at": "2024-01-15T14:30:00Z",
  "updated_at": "2024-01-16T10:00:00Z"
}
```

**Error Responses:**
- `400 Bad Request` - Validation failed
- `404 Not Found` - Expense or category not found

---

### Delete Expense

Deletes an expense.

**Endpoint:** `DELETE /expenses/:id`

**Headers:**
```
Authorization: Bearer <token>
```

**URL Parameters:**
- `id` - Expense UUID

**Response:** `204 No Content`

**Error Responses:**
- `404 Not Found` - Expense not found

---

## Summaries

### Monthly Summary

Returns expense totals grouped by month for the last 12 months.

**Endpoint:** `GET /summaries/monthly`

**Headers:**
```
Authorization: Bearer <token>
```

**Response:** `200 OK`
```json
[
  {
    "month": "January",
    "year": 2024,
    "total_amount": "1523.45",
    "expense_count": 42
  },
  {
    "month": "December",
    "year": 2023,
    "total_amount": "1834.20",
    "expense_count": 38
  }
]
```

**Notes:**
- Returns up to 12 most recent months
- Ordered by year and month descending (most recent first)
- Months with no expenses are not included

---

### Category Summary

Returns expense totals grouped by category for the current month.

**Endpoint:** `GET /summaries/categories`

**Headers:**
```
Authorization: Bearer <token>
```

**Response:** `200 OK`
```json
[
  {
    "category_id": "cat-uuid-1",
    "category_name": "Food & Dining",
    "category_color": "#FF6B6B",
    "category_icon": "🍔",
    "total_amount": "450.25",
    "expense_count": 15
  },
  {
    "category_id": "cat-uuid-2",
    "category_name": "Transportation",
    "category_color": "#4ECDC4",
    "category_icon": "🚗",
    "total_amount": "280.00",
    "expense_count": 8
  }
]
```

**Notes:**
- Includes all user categories (even those with zero expenses)
- Ordered by total_amount descending (highest spending first)
- Only counts expenses from current month

---

## Error Responses

All error responses follow this format:

```json
{
  "error": "Error message description"
}
```

### HTTP Status Codes

| Code | Meaning | Common Causes |
|------|---------|---------------|
| `200` | OK | Successful GET/PUT request |
| `201` | Created | Successful POST request |
| `204` | No Content | Successful DELETE request |
| `400` | Bad Request | Validation failed, invalid input |
| `401` | Unauthorized | Missing/invalid/expired token |
| `404` | Not Found | Resource doesn't exist |
| `500` | Internal Server Error | Server error (check logs) |

### Common Error Messages

```json
// Authentication errors
{
  "error": "Missing authorization header"
}
{
  "error": "Invalid or expired token"
}
{
  "error": "Invalid credentials"
}

// Validation errors
{
  "error": "Invalid email address"
}
{
  "error": "Password must be at least 8 characters"
}
{
  "error": "Amount must be greater than 0"
}

// Resource errors
{
  "error": "Category not found"
}
{
  "error": "Expense not found"
}
{
  "error": "Email already registered"
}

// Business logic errors
{
  "error": "Cannot delete category with existing expenses"
}
```

---

## Request Examples

### Using cURL

#### Register User
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "full_name": "John Doe"
  }'
```

#### Login
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

#### List Expenses (with auth)
```bash
curl -X GET http://localhost:3000/api/expenses \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

#### Create Expense
```bash
curl -X POST http://localhost:3000/api/expenses \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": "cat-uuid",
    "amount": 42.50,
    "description": "Lunch",
    "expense_date": "2024-01-15"
  }'
```

#### Filter Expenses by Date
```bash
curl -X GET "http://localhost:3000/api/expenses?start_date=2024-01-01&end_date=2024-01-31" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

---

### Using JavaScript (Fetch API)

```javascript
// Register
const response = await fetch('http://localhost:3000/api/auth/register', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    email: 'user@example.com',
    password: 'password123',
    full_name: 'John Doe'
  })
});
const data = await response.json();
const token = data.token;

// Create expense (authenticated)
const expenseResponse = await fetch('http://localhost:3000/api/expenses', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    category_id: 'cat-uuid',
    amount: 42.50,
    description: 'Lunch',
    expense_date: '2024-01-15'
  })
});
```

---

### Using Python (requests)

```python
import requests

# Register
response = requests.post(
    'http://localhost:3000/api/auth/register',
    json={
        'email': 'user@example.com',
        'password': 'password123',
        'full_name': 'John Doe'
    }
)
token = response.json()['token']

# List expenses
response = requests.get(
    'http://localhost:3000/api/expenses',
    headers={'Authorization': f'Bearer {token}'}
)
expenses = response.json()
```

---

## Rate Limiting

Currently, there is no rate limiting implemented. For production use, consider adding:

- Rate limiting per IP
- Rate limiting per user
- Request throttling for expensive operations

---

## Versioning

Current API version: `v1` (implicit in base path)

Future versions will use URL versioning:
- `/api/v1/...` - Current version
- `/api/v2/...` - Future version

---

## Security Considerations

### Best Practices

1. **Always use HTTPS in production**
2. **Store JWT tokens securely** (HttpOnly cookies or secure storage)
3. **Never expose JWT_SECRET**
4. **Implement rate limiting**
5. **Validate all input on both client and server**
6. **Use CORS properly** (don't use `allow_origin(Any)` in production)
7. **Log security events** (failed logins, token expiration, etc.)

### Token Management

- Tokens expire after 24 hours (configurable)
- No refresh token mechanism yet
- Users must re-login after expiration
- Tokens are stateless (stored only on client)

---

## Testing the API

### Health Check
```bash
curl http://localhost:3000/health
# Should return: OK
```

### Complete Workflow

```bash
# 1. Register
TOKEN=$(curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","password":"test1234","full_name":"Test User"}' \
  | jq -r '.token')

# 2. Get user info
curl http://localhost:3000/api/users/me \
  -H "Authorization: Bearer $TOKEN"

# 3. List categories
curl http://localhost:3000/api/categories \
  -H "Authorization: Bearer $TOKEN"

# 4. Create expense
curl -X POST http://localhost:3000/api/expenses \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id":"<cat-id-from-step-3>",
    "amount":50.00,
    "description":"Test expense",
    "expense_date":"2024-01-15"
  }'

# 5. List expenses
curl http://localhost:3000/api/expenses \
  -H "Authorization: Bearer $TOKEN"

# 6. Get summaries
curl http://localhost:3000/api/summaries/categories \
  -H "Authorization: Bearer $TOKEN"
```

---

**For more information, see the main [README.md](README.md) and [SETUP.md](SETUP.md).**
