use leptos::*;

use crate::api::{login, register};
use crate::models::{LoginRequest, RegisterRequest};

#[component]
pub fn Auth<F>(on_auth: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let (is_login, set_is_login) = create_signal(true);
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (full_name, set_full_name) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);
        set_loading.set(true);

        let email_val = email.get();
        let password_val = password.get();
        let full_name_val = full_name.get();
        let is_login_val = is_login.get();

        spawn_local(async move {
            let result = if is_login_val {
                login(LoginRequest {
                    email: email_val,
                    password: password_val,
                })
                .await
            } else {
                register(RegisterRequest {
                    email: email_val,
                    password: password_val,
                    full_name: full_name_val,
                })
                .await
            };

            set_loading.set(false);

            match result {
                Ok(_) => {
                    on_auth();
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
        });
    };

    view! {
        <div class="auth-container">
            <div class="card">
                <h1 style="text-align: center; margin-bottom: 24px; color: #667eea;">
                    {move || if is_login.get() { "Login" } else { "Register" }}
                </h1>

                {move || error.get().map(|e| view! {
                    <div class="error">{e}</div>
                })}

                <form on:submit=handle_submit>
                    <div class="form-group">
                        <label>"Email"</label>
                        <input
                            type="email"
                            required
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            placeholder="Enter your email"
                        />
                    </div>

                    <div class="form-group">
                        <label>"Password"</label>
                        <input
                            type="password"
                            required
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            placeholder="Enter your password"
                        />
                    </div>

                    {move || (!is_login.get()).then(|| view! {
                        <div class="form-group">
                            <label>"Full Name"</label>
                            <input
                                type="text"
                                required
                                prop:value=full_name
                                on:input=move |ev| set_full_name.set(event_target_value(&ev))
                                placeholder="Enter your full name"
                            />
                        </div>
                    })}

                    <div class="form-group">
                        <button
                            type="submit"
                            disabled=loading
                            style="width: 100%;"
                        >
                            {move || if loading.get() { "Loading..." } else if is_login.get() { "Login" } else { "Register" }}
                        </button>
                    </div>
                </form>

                <p style="text-align: center; margin-top: 16px; color: #6c757d;">
                    {move || if is_login.get() {
                        "Don't have an account? "
                    } else {
                        "Already have an account? "
                    }}
                    <a
                        href="#"
                        on:click=move |ev| {
                            ev.prevent_default();
                            set_is_login.update(|v| *v = !*v);
                            set_error.set(None);
                        }
                        style="color: #667eea; font-weight: 600;"
                    >
                        {move || if is_login.get() { "Register" } else { "Login" }}
                    </a>
                </p>
            </div>
        </div>
    }
}
