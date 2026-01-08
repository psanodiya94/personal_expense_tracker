use chrono::{Datelike, Local};
use leptos::*;
use uuid::Uuid;

use crate::api::{
    clear_token, delete_expense, get_category_summary, get_monthly_summary, list_categories,
    list_expenses,
};
use crate::models::{Category, CategorySummary, Expense, MonthlySummary};

#[component]
pub fn Dashboard(on_logout: WriteSignal<bool>) -> impl IntoView {
    let (categories, set_categories) = create_signal(Vec::<Category>::new());
    let (expenses, set_expenses) = create_signal(Vec::<Expense>::new());
    let (monthly_summary, set_monthly_summary) = create_signal(Vec::<MonthlySummary>::new());
    let (category_summary, set_category_summary) = create_signal(Vec::<CategorySummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    let (filter_category, set_filter_category) = create_signal(None::<Uuid>);
    let (filter_start_date, set_filter_start_date) = create_signal(None::<String>);
    let (filter_end_date, set_filter_end_date) = create_signal(None::<String>);

    let reload_data = create_rw_signal(0);

    create_effect(move |_| {
        reload_data.get();
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match list_categories().await {
                Ok(cats) => set_categories.set(cats),
                Err(e) => set_error.set(Some(e)),
            }

            let start = filter_start_date.get();
            let end = filter_end_date.get();
            let cat = filter_category.get();

            match list_expenses(start, end, cat).await {
                Ok(exps) => set_expenses.set(exps),
                Err(e) => set_error.set(Some(e)),
            }

            match get_monthly_summary().await {
                Ok(summary) => set_monthly_summary.set(summary),
                Err(e) => set_error.set(Some(e)),
            }

            match get_category_summary().await {
                Ok(summary) => set_category_summary.set(summary),
                Err(e) => set_error.set(Some(e)),
            }

            set_loading.set(false);
        });
    });

    let handle_logout = move |_| {
        clear_token();
        on_logout.set(true);
    };

    let handle_delete = move |id: Uuid| {
        spawn_local(async move {
            match delete_expense(id).await {
                Ok(_) => reload_data.update(|v| *v += 1),
                Err(e) => set_error.set(Some(e)),
            }
        });
    };

    let total_this_month = move || {
        let now = Local::now();
        expenses
            .get()
            .iter()
            .filter(|e| {
                e.expense_date.year() == now.year() && e.expense_date.month() == now.month()
            })
            .map(|e| e.amount)
            .sum::<f64>()
    };

    let expense_count_this_month = move || {
        let now = Local::now();
        expenses
            .get()
            .iter()
            .filter(|e| {
                e.expense_date.year() == now.year() && e.expense_date.month() == now.month()
            })
            .count()
    };

    view! {
        <div class="container">
            <div class="header">
                <h1>"Expense Tracker"</h1>
                <button on:click=handle_logout class="btn-secondary">
                    "Logout"
                </button>
            </div>

            {move || error.get().map(|e| view! {
                <div class="error">{e}</div>
            })}

            {move || if loading.get() {
                view! { <div class="loading">"Loading..."</div> }.into_view()
            } else {
                view! {
                    <div>
                        <div class="summary-grid">
                            <div class="summary-card">
                                <h3>"This Month"</h3>
                                <div class="value">"$"{format!("{:.2}", total_this_month())}</div>
                            </div>
                            <div class="summary-card">
                                <h3>"Expenses Count"</h3>
                                <div class="value">{expense_count_this_month()}</div>
                            </div>
                            <div class="summary-card">
                                <h3>"Categories"</h3>
                                <div class="value">{move || categories.get().len()}</div>
                            </div>
                        </div>

                        <crate::components::expense_form::ExpenseForm
                            categories=categories
                            on_created=WriteSignal::from(move |_| reload_data.update(|v| *v += 1))
                        />

                        <div class="card">
                            <h2 style="margin-bottom: 20px; color: #333;">"Filters"</h2>
                            <div class="filters">
                                <div class="form-group">
                                    <label>"Category"</label>
                                    <select on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        if value.is_empty() {
                                            set_filter_category.set(None);
                                        } else if let Ok(uuid) = Uuid::parse_str(&value) {
                                            set_filter_category.set(Some(uuid));
                                        }
                                        reload_data.update(|v| *v += 1);
                                    }>
                                        <option value="">"All Categories"</option>
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
                                    <label>"Start Date"</label>
                                    <input
                                        type="date"
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            set_filter_start_date.set(if value.is_empty() { None } else { Some(value) });
                                            reload_data.update(|v| *v += 1);
                                        }
                                    />
                                </div>

                                <div class="form-group">
                                    <label>"End Date"</label>
                                    <input
                                        type="date"
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            set_filter_end_date.set(if value.is_empty() { None } else { Some(value) });
                                            reload_data.update(|v| *v += 1);
                                        }
                                    />
                                </div>
                            </div>
                        </div>

                        <div class="card">
                            <h2 style="margin-bottom: 20px; color: #333;">"Recent Expenses"</h2>
                            <div class="expense-list">
                                {move || {
                                    let exps = expenses.get();
                                    if exps.is_empty() {
                                        view! {
                                            <p style="text-align: center; color: #6c757d; padding: 20px;">
                                                "No expenses found. Add your first expense above!"
                                            </p>
                                        }.into_view()
                                    } else {
                                        exps.into_iter().map(|expense| {
                                            let color = expense.category_color.clone().unwrap_or_else(|| "#667eea".to_string());
                                            let exp_id = expense.id;
                                            view! {
                                                <div class="expense-item" style:border-left-color=color>
                                                    <div class="expense-icon">
                                                        {expense.category_icon.unwrap_or_else(|| "📦".to_string())}
                                                    </div>
                                                    <div class="expense-details">
                                                        <h3>{&expense.description}</h3>
                                                        <p>{expense.category_name.clone()} " • " {expense.expense_date.format("%b %d, %Y").to_string()}</p>
                                                    </div>
                                                    <div class="expense-amount">
                                                        "$"{format!("{:.2}", expense.amount)}
                                                    </div>
                                                    <div class="expense-actions">
                                                        <button
                                                            class="btn-danger"
                                                            on:click=move |_| {
                                                                if web_sys::window()
                                                                    .and_then(|w| w.confirm_with_message("Delete this expense?").ok())
                                                                    .unwrap_or(false)
                                                                {
                                                                    handle_delete(exp_id);
                                                                }
                                                            }
                                                        >
                                                            "Delete"
                                                        </button>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>().into_view()
                                    }
                                }}
                            </div>
                        </div>

                        <div class="card">
                            <h2 style="margin-bottom: 20px; color: #333;">"Category Summary (This Month)"</h2>
                            <div class="expense-list">
                                {move || category_summary.get().into_iter().map(|summary| {
                                    let color = summary.category_color.clone().unwrap_or_else(|| "#667eea".to_string());
                                    view! {
                                        <div class="expense-item" style:border-left-color=color>
                                            <div class="expense-icon">
                                                {summary.category_icon.unwrap_or_else(|| "📦".to_string())}
                                            </div>
                                            <div class="expense-details">
                                                <h3>{&summary.category_name}</h3>
                                                <p>{summary.expense_count} " expenses"</p>
                                            </div>
                                            <div class="expense-amount">
                                                "$"{format!("{:.2}", summary.total_amount)}
                                            </div>
                                            <div></div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                }.into_view()
            }}
        </div>
    }
}
