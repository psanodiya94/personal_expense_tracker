use chrono::Local;
use leptos::*;
use uuid::Uuid;

use crate::api::create_expense;
use crate::models::{Category, CreateExpense};

#[component]
pub fn ExpenseForm<F>(
    categories: ReadSignal<Vec<Category>>,
    on_created: F,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let (category_id, set_category_id) = create_signal(None::<Uuid>);
    let (amount, set_amount) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (expense_date, set_expense_date) = create_signal(
        Local::now().format("%Y-%m-%d").to_string()
    );
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);

        let Some(cat_id) = category_id.get() else {
            set_error.set(Some("Please select a category".to_string()));
            return;
        };

        let amount_val = match amount.get().parse::<f64>() {
            Ok(v) if v > 0.0 => v,
            _ => {
                set_error.set(Some("Please enter a valid amount".to_string()));
                return;
            }
        };

        let desc = description.get();
        if desc.trim().is_empty() {
            set_error.set(Some("Please enter a description".to_string()));
            return;
        }

        let date_str = expense_date.get();
        let date = match chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                set_error.set(Some("Invalid date format".to_string()));
                return;
            }
        };

        set_loading.set(true);

        spawn_local(async move {
            let result = create_expense(CreateExpense {
                category_id: cat_id,
                amount: amount_val,
                description: desc.clone(),
                expense_date: date,
            })
            .await;

            set_loading.set(false);

            match result {
                Ok(_) => {
                    set_amount.set(String::new());
                    set_description.set(String::new());
                    set_category_id.set(None);
                    on_created();
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
        });
    };

    view! {
        <div class="card">
            <h2 style="margin-bottom: 20px; color: #333;">
                "Add New Expense"
            </h2>

            {move || error.get().map(|e| view! {
                <div class="error">{e}</div>
            })}

            <form on:submit=handle_submit>
                <div class="form-group">
                    <label>"Category"</label>
                    <select
                        required
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(uuid) = Uuid::parse_str(&value) {
                                set_category_id.set(Some(uuid));
                            }
                        }
                    >
                        <option value="">"Select a category"</option>
                        {move || categories.get().into_iter().map(|cat| {
                            view! {
                                <option value={cat.id.to_string()}>
                                    {cat.icon.as_ref().map(|i| format!("{} ", i)).unwrap_or_default()}
                                    {&cat.name}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>

                <div class="form-group">
                    <label>"Amount ($)"</label>
                    <input
                        type="number"
                        step="0.01"
                        min="0.01"
                        required
                        prop:value=amount
                        on:input=move |ev| set_amount.set(event_target_value(&ev))
                        placeholder="0.00"
                    />
                </div>

                <div class="form-group">
                    <label>"Description"</label>
                    <input
                        type="text"
                        required
                        prop:value=description
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        placeholder="What did you spend on?"
                    />
                </div>

                <div class="form-group">
                    <label>"Date"</label>
                    <input
                        type="date"
                        required
                        prop:value=expense_date
                        on:input=move |ev| set_expense_date.set(event_target_value(&ev))
                    />
                </div>

                <button type="submit" disabled=loading style="width: 100%;">
                    {move || if loading.get() { "Adding..." } else { "Add Expense" }}
                </button>
            </form>
        </div>
    }
}
