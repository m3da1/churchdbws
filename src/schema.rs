table! {
    attendance (id) {
        id -> Integer,
        member_id -> Integer,
        attend_date -> Date,
        created_date -> Nullable<Datetime>,
        modified_date -> Nullable<Datetime>,
        status -> Nullable<Varchar>,
    }
}

table! {
    members (id) {
        id -> Integer,
        surname -> Varchar,
        firstname -> Varchar,
        othernames -> Nullable<Varchar>,
        dob -> Nullable<Date>,
        gender -> Nullable<Varchar>,
        maritalstatus -> Nullable<Varchar>,
        employed -> Nullable<Varchar>,
        occupation -> Nullable<Varchar>,
        company -> Nullable<Varchar>,
        companylocation -> Nullable<Varchar>,
        residence -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        passport -> Nullable<Varchar>,
        datecreated -> Nullable<Datetime>,
        status -> Nullable<Varchar>,
        modified_date -> Nullable<Datetime>,
        presbytery -> Nullable<Varchar>,
    }
}

table! {
    stewardgroups (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        leader -> Integer,
        date_created -> Datetime,
        status -> Nullable<Varchar>,
        modified_date -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        created_by -> Nullable<Integer>,
        datecreated -> Nullable<Datetime>,
        msisdn -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        lastlogindate -> Nullable<Datetime>,
        lastupdatetime -> Nullable<Datetime>,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
    }
}

table! {
    zonalgroups (id) {
        id -> Integer,
        name -> Varchar,
        leader -> Integer,
        groups -> Varchar,
        date_created -> Datetime,
        status -> Varchar,
        modified_created -> Nullable<Datetime>,
    }
}

allow_tables_to_appear_in_same_query!(
    attendance,
    members,
    stewardgroups,
    users,
    zonalgroups,
);
