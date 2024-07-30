LET $workbucket = fn::create_bucket('work', '#ff0000');
LET $homebucket = fn::create_bucket('home', '#00ff00');

LET $msauth = fn::create_twofactor('Microsoft Authentification', 'A70');

LET $msacc = fn::create_supersecure_account('tobiashoelzer@hotmail.com', 'tech', 1111, 10, 26, 'Microsoft', '3MTMwMzQ3ODU', 'https://microsoft.com', none, $homebucket, $msauth);
fn::create_secure_account('tobiashoelzer@hotmail.com', 'games', 'Faceit', none, none, none, $homebucket, none);
fn::create_secure_account('tobiashoelzer@hotmail.com', 'social', 'Instagram', none, none, none, $homebucket, none);
fn::create_sso_account($msacc, 'GitHub', none, 'https://github.com', ['GutHub', 'Repos'], $homebucket, none);

fn::create_secure_account('tobiashoelzer@work.com', 'tech', 'Slack', none, none, none, $workbucket, none);
