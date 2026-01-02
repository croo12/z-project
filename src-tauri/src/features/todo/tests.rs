#[cfg(test)]
mod tests {
    use super::super::model::Todo;

    #[test]
    fn test_todo_struct() {
        let todo = Todo {
            id: 1,
            text: "Test".to_string(),
            completed: false,
        };
        assert_eq!(todo.id, 1);
        assert_eq!(todo.text, "Test");
        assert!(!todo.completed);
    }
}
