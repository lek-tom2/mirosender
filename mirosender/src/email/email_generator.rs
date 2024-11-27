use crate::constants::EMAIL_CSS_TEMPLATE;

pub enum EmailType {
    HomeWorkCodeAsText(String),
    HomeWorkCodeAsAttachment,
}

pub fn email_body_generator(email_type: EmailType) -> String {
    match email_type {
        EmailType::HomeWorkCodeAsText(code) => {
            return format!(
                r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
    {}
    </style>
</head>
<body>

<div class="container">
    <header class="header">
        <h1>Welcome to Electro-Vision</h1>
    </header>

    <section class="body">
        <article>
            <p>Hello Mr. Moscicki!</p>
            <p>Hope I get 6</p>
            <div style="text-align: center;">
            <p>{}</p>
            </div>
            <p>Best Regards</p>
        </article>
    </section>

</div>

</body>
</html>
                "#,
                EMAIL_CSS_TEMPLATE, code
            );
        }
        EmailType::HomeWorkCodeAsAttachment => {
            return format!(
                r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
    {}
    </style>
</head>
<body>

<div class="container">
    <header class="header">
        <h1>Welcome to Electro-Vision</h1>
    </header>

    <section class="body">
        <article>
            <p>Hello Mr. Moscicki!</p>
            <p>Hope I get 6</p>
            <div style="text-align: center;">
            <p>Code is in attachment</p>
            </div>
            <p>Best Regards</p>
        </article>
    </section>

</div>

</body>
</html>
                "#,
                EMAIL_CSS_TEMPLATE
            );
        }
    }
}
