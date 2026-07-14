// This trait will replace the current implementation.
// Right now, the implementation is locked on only SQLite.
// With this trait, we can implement other savers like Postgres, MySQL, etc.
// Or even third party services like Hashicorp Vault, AWS Secrets Manager, etc.

// For now, I'll just get an MVP in place.
pub trait Store {}
