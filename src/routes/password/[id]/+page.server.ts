enum Industry {
  Tech,     // -> @
  Games,    // -> !
  Social,   // -> #
  Finance,  // -> $
  Shopping, // -> *
  Science,  // -> ?
  Other     // -> &
}

interface  Acc {
    id: number;
    account_name: string, // Users username
    secret: string, // Users secret
    institution: string,
    industry: Industry,
    is_legacy: boolean,
    is_work: boolean, // Make this rather buckets of Accounts -> Bucket for IBM, Bucket for DHBW, Bucket for Private...
    date_created: Date
}


export function load({params}) {

  const acc: Acc = {
    id: parseInt(params.id),
    account_name: "Acc Uname", // Users username
    secret: "Acc Secret", // Users secret
    institution: "Science",
    industry: Industry.Science,
    is_legacy: false,
    is_work: false,
    date_created: new Date()
  }
  return acc;
}
