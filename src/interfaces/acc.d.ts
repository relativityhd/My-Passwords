export enum Industry {
  Tech,     // -> @
  Games,    // -> !
  Social,   // -> #
  Finance,  // -> $
  Shopping, // -> *
  Science,  // -> ?
  Other     // -> &
}

export interface  Acc {
    id: number;
    account_name: string, // Users username
    secret: string, // Users secret
    institution: string,
    industry: Industry,
    is_legacy: bool,
    is_work: bool, // Make this rather buckets of Accounts -> Bucket for IBM, Bucket for DHBW, Bucket for Private...
    date_created: DateTime<Utc>
}
