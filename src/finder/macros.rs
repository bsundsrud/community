
macro_rules! int_range {
    ($field:expr, $s:ident, $e:ident) => (
        if let Some(x) = $field {
            if x >= $s && x <= $e {
                Status::Met
            } else {
                Status::NotMet
            }
        } else {
            Status::Unknown
        }
    )
}

macro_rules! int_equals {
    ($field:expr, $i:ident) => (
        if let Some(i) = $field {
            if $i == i {
                Status::Met
            } else {
                Status::NotMet
            }
        } else {
            Status::Unknown
        }
    )
}

macro_rules! string_equals {
    ($field:expr, $s:ident) => (
        if let Some(ref s) = $field {
            if $s == s {
                Status::Met
            } else {
                Status::NotMet
            }
        } else {
            Status::Unknown
        }
    )
}

macro_rules! boolean {
    ($field:expr, $b:ident) => (
        if let Some(b) = $field {
            if $b == b {
                Status::Met
            } else {
                Status::NotMet
            }
        } else {
            Status::Unknown
        }
    )
}

macro_rules! requirements {
    ($self_:ident, $( $p:pat => $e: expr ),+ ) => (
        match $self_.req_type {
            $($p => $e),+,
            _ => { unimplemented!(); }
        }
    )
}

macro_rules! fields {
    ($self_:ident, $( $field:pat => $e:expr ),+ ) => (
        match $self_.field {
            $($field => $e),+,
        }
    )
}
