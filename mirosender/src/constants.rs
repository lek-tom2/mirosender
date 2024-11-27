pub const TEACHER_EMAIL: &str = "miroslawmosciki@gmai.com";
pub const EMAIL_CSS_TEMPLATE: &str = r#"
    body, html {
        margin: 0;
        padding: 0;
        font-family: Arial, sans-serif;
        background-color: #f3f3f3;
    }
    .container {
        max-width: 600px;
        margin: 20px auto;
        background-color: #ffffff;
        border-radius: 8px;
        overflow: hidden;
        box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.1);
    }
    .header {
        background-color: #003399;
        color: #ffffff;
        padding: 20px;
        text-align: center;
    }
    .header h1 {
        margin: 0;
    }
    .body {
        padding: 20px;
        color: #333333;
        line-height: 1.6;
    }
    .body p {
        margin: 10px 0;
    }
    .verify-button {
        display: inline-block;
        margin: 20px 0;
        padding: 12px 24px;
        background-color: #003399;
        color: #ffffff;
        text-decoration: none;
        border-radius: 6px;
        font-weight: bold;
        text-align: center;
    }
    .verify-button:hover {
        background-color: #002277;
    }
    .footer {
        padding: 15px;
        text-align: center;
        font-size: 12px;
        color: #777777;
        background-color: #f3f3f3;
    }
    .footer p {
        margin: 5px 0;
    }
    /* Responsive styling */
    @media only screen and (max-width: 600px) {
        .container {
            width: 100%;
        }
        .header, .body, .footer {
            padding: 10px;
        }
        .verify-button {
            width: 100%;
            box-sizing: border-box;
        }
    }
"#;
