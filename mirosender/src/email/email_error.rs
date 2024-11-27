pub enum EmailError {
    SenderEmailInvalid,
    SenderEmailNotFound,
    ReceiverEmailInvalid,
    ReceiverEmailNotFound,
    NoTopic,
    AttachmentInvalid,
    AttachmentNotFound,
    PasswordInvalid,
    UnexpectedError,
}
