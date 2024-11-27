use surrealsdk::*;

pub fn upsert<T: Table>(q: &mut Query, condition: impl Into<Query>, content: impl Into<Query>) {
    let condition = condition.into();
    let content = content.into();
    q.m(transaction());
    q.m(format!("LET $found = (SELECT * FROM {}", T::table()));
    q.m(condition.clone());
    q.m("LIMIT 1).len() == 1");
    q.end();
    q.m("IF ($found) {");
    q.m(format!("LET $r = (UPDATE {} CONTENT", T::table()));
    q.m(content.clone());
    q.m(condition);
    q.m(")");
    q.end();
    q.m("RETURN $r;");
    q.m("}");
    q.m("ELSE {");
    q.m(format!("LET $r = (CREATE {} CONTENT", T::table()));
    q.m(content);
    q.m(")");
    q.end();
    q.m("RETURN $r;");
    q.m("}");
    q.end();
    q.m("COMMIT TRANSACTION;\n");
}

#[cfg(test)]
mod test {
    use crate::model::user::User;

    use super::*;
    use surrealsdk::*;

    #[test]
    fn test_upsert() {
        let mut q = Query::default();
        upsert::<User>(
            &mut q,
            "WHERE id = $id".q().bind("id", "1"),
            "$content".q().bind("content", 1),
        );
        dbg!(&q);
        println!("{}", q);
    }
}
