// This file was generated automatically.
// See the gen/ folder at the project root.

use std::fmt;
use std::str;

/// Representation of IRC commands, replies and errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Code {
    /// PASS = "PASS"
    Pass,
    /// NICK = "NICK"
    Nick,
    /// USER = "USER"
    User,
    /// OPER = "OPER"
    Oper,
    /// MODE = "MODE"
    Mode,
    /// SERVICE = "SERVICE"
    Service,
    /// QUIT = "QUIT"
    Quit,
    /// SQUIT = "SQUIT"
    Squit,
    /// JOIN = "JOIN"
    Join,
    /// PART = "PART"
    Part,
    /// TOPIC = "TOPIC"
    Topic,
    /// NAMES = "NAMES"
    Names,
    /// LIST = "LIST"
    List,
    /// INVITE = "INVITE"
    Invite,
    /// KICK = "KICK"
    Kick,
    /// PRIVMSG = "PRIVMSG"
    Privmsg,
    /// NOTICE = "NOTICE"
    Notice,
    /// MOTD = "MOTD"
    Motd,
    /// LUSERS = "LUSERS"
    Lusers,
    /// VERSION = "VERSION"
    Version,
    /// STATS = "STATS"
    Stats,
    /// LINKS = "LINKS"
    Links,
    /// TIME = "TIME"
    Time,
    /// CONNECT = "CONNECT"
    Connect,
    /// TRACE = "TRACE"
    Trace,
    /// ADMIN = "ADMIN"
    Admin,
    /// INFO = "INFO"
    Info,
    /// SERVLIST = "SERVLIST"
    Servlist,
    /// SQUERY = "SQUERY"
    Squery,
    /// WHO = "WHO"
    Who,
    /// WHOIS = "WHOIS"
    Whois,
    /// WHOWAS = "WHOWAS"
    Whowas,
    /// KILL = "KILL"
    Kill,
    /// PING = "PING"
    Ping,
    /// PONG = "PONG"
    Pong,
    /// ERROR = "ERROR"
    Error,
    /// AWAY = "AWAY"
    Away,
    /// REHASH = "REHASH"
    Rehash,
    /// DIE = "DIE"
    Die,
    /// RESTART = "RESTART"
    Restart,
    /// SUMMON = "SUMMON"
    Summon,
    /// USERS = "USERS"
    Users,
    /// WALLOPS = "WALLOPS"
    Wallops,
    /// USERHOST = "USERHOST"
    Userhost,
    /// ISON = "ISON"
    Ison,
    /// RPL_WELCOME = "001"
    RplWelcome,
    /// RPL_YOURHOST = "002"
    RplYourhost,
    /// RPL_CREATED = "003"
    RplCreated,
    /// RPL_MYINFO = "004"
    RplMyinfo,
    /// RPL_BOUNCE = "005"
    RplBounce,
    /// RPL_USERHOST = "302"
    RplUserhost,
    /// RPL_ISON = "303"
    RplIson,
    /// RPL_AWAY = "301"
    RplAway,
    /// RPL_UNAWAY = "305"
    RplUnaway,
    /// RPL_NOWAWAY = "306"
    RplNowaway,
    /// RPL_WHOISUSER = "311"
    RplWhoisuser,
    /// RPL_WHOISSERVER = "312"
    RplWhoisserver,
    /// RPL_WHOISOPERATOR = "313"
    RplWhoisoperator,
    /// RPL_WHOISIDLE = "317"
    RplWhoisidle,
    /// RPL_ENDOFWHOIS = "318"
    RplEndofwhois,
    /// RPL_WHOISCHANNELS = "319"
    RplWhoischannels,
    /// RPL_WHOWASUSER = "314"
    RplWhowasuser,
    /// RPL_ENDOFWHOWAS = "369"
    RplEndofwhowas,
    /// RPL_LISTSTART = "321"
    RplListstart,
    /// RPL_LIST = "322"
    RplList,
    /// RPL_LISTEND = "323"
    RplListend,
    /// RPL_UNIQOPIS = "325"
    RplUniqopis,
    /// RPL_CHANNELMODEIS = "324"
    RplChannelmodeis,
    /// RPL_NOTOPIC = "331"
    RplNotopic,
    /// RPL_TOPIC = "332"
    RplTopic,
    /// RPL_INVITING = "341"
    RplInviting,
    /// RPL_SUMMONING = "342"
    RplSummoning,
    /// RPL_INVITELIST = "346"
    RplInvitelist,
    /// RPL_ENDOFINVITELIST = "347"
    RplEndofinvitelist,
    /// RPL_EXCEPTLIST = "348"
    RplExceptlist,
    /// RPL_ENDOFEXECPTLIST = "349"
    RplEndofexecptlist,
    /// RPL_VERSION = "351"
    RplVersion,
    /// RPL_WHOREPLY = "352"
    RplWhoreply,
    /// RPL_ENDOFWHO = "315"
    RplEndofwho,
    /// RPL_NAMREPLY = "353"
    RplNamreply,
    /// RPL_ENDOFNAMES = "366"
    RplEndofnames,
    /// RPL_LINKS = "364"
    RplLinks,
    /// RPL_ENDOFLINKS = "365"
    RplEndoflinks,
    /// RPL_BANLIST = "367"
    RplBanlist,
    /// RPL_ENDOFBANLIST = "368"
    RplEndofbanlist,
    /// RPL_INFO = "371"
    RplInfo,
    /// RPL_ENDOFINFO = "374"
    RplEndofinfo,
    /// RPL_MOTDSTART = "375"
    RplMotdstart,
    /// RPL_MOTD = "372"
    RplMotd,
    /// RPL_ENDOFMOTD = "376"
    RplEndofmotd,
    /// RPL_YOUREOPER = "381"
    RplYoureoper,
    /// RPL_REHASHING = "382"
    RplRehashing,
    /// RPL_YOURESERVICE = "383"
    RplYoureservice,
    /// RPL_TIME = "391"
    RplTime,
    /// RPL_USERSSTART = "392"
    RplUsersstart,
    /// RPL_USERS = "393"
    RplUsers,
    /// RPL_ENDOFUSERS = "394"
    RplEndofusers,
    /// RPL_NOUSERS = "395"
    RplNousers,
    /// RPL_TRACELINK = "200"
    RplTracelink,
    /// RPL_TRACECONNECTING = "201"
    RplTraceconnecting,
    /// RPL_TRACEHANDSHAKE = "202"
    RplTracehandshake,
    /// RPL_TRACEUKNOWN = "203"
    RplTraceuknown,
    /// RPL_TRACEOPERATOR = "204"
    RplTraceoperator,
    /// RPL_TRACEUSER = "205"
    RplTraceuser,
    /// RPL_TRACESERVER = "206"
    RplTraceserver,
    /// RPL_TRACESERVICE = "207"
    RplTraceservice,
    /// RPL_TRACENEWTYPE = "208"
    RplTracenewtype,
    /// RPL_TRACECLASS = "209"
    RplTraceclass,
    /// RPL_TRACERECONNECT = "210"
    RplTracereconnect,
    /// RPL_TRACELOG = "261"
    RplTracelog,
    /// RPL_TRACEEND = "262"
    RplTraceend,
    /// RPL_STATSLINKINFO = "211"
    RplStatslinkinfo,
    /// RPL_STATSCOMMANDS = "212"
    RplStatscommands,
    /// RPL_ENDOFSTATS = "219"
    RplEndofstats,
    /// RPL_STATSUPTIME = "242"
    RplStatsuptime,
    /// RPL_STATSOLINE = "243"
    RplStatsoline,
    /// RPL_UMODEIS = "221"
    RplUmodeis,
    /// RPL_SERVLIST = "234"
    RplServlist,
    /// RPL_SERVLISTEND = "235"
    RplServlistend,
    /// RPL_LUSERCLIENT = "251"
    RplLuserclient,
    /// RPL_LUSEROP = "252"
    RplLuserop,
    /// RPL_LUSERUNKNOWN = "253"
    RplLuserunknown,
    /// RPL_LUSERCHANNELS = "254"
    RplLuserchannels,
    /// RPL_LUSERME = "255"
    RplLuserme,
    /// RPL_ADMINME = "256"
    RplAdminme,
    /// RPL_ADMINLOC1 = "257"
    RplAdminloc1,
    /// RPL_ADMINLOC2 = "258"
    RplAdminloc2,
    /// RPL_ADMINEMAIL = "259"
    RplAdminemail,
    /// RPL_TRYAGAIN = "263"
    RplTryagain,
    /// ERR_NOSUCHNICK = "401"
    ErrNosuchnick,
    /// ERR_NOSUCHSERVER = "402"
    ErrNosuchserver,
    /// ERR_NOSUCHCHANNEL = "403"
    ErrNosuchchannel,
    /// ERR_CANNOTSENDTOCHAN = "404"
    ErrCannotsendtochan,
    /// ERR_TOOMANYCHANNELS = "405"
    ErrToomanychannels,
    /// ERR_WASNOSUCHNICK = "406"
    ErrWasnosuchnick,
    /// ERR_TOOMANYTARGETS = "407"
    ErrToomanytargets,
    /// ERR_NOSUCHSERVICE = "408"
    ErrNosuchservice,
    /// ERR_NOORIGIN = "409"
    ErrNoorigin,
    /// ERR_NORECIPIENT = "411"
    ErrNorecipient,
    /// ERR_NOTEXTTOSEND = "412"
    ErrNotexttosend,
    /// ERR_NOTOPLEVEL = "413"
    ErrNotoplevel,
    /// ERR_WILDTOPLEVEL = "414"
    ErrWildtoplevel,
    /// ERR_BADMASK = "415"
    ErrBadmask,
    /// ERR_UNKNOWNCOMMAND = "421"
    ErrUnknowncommand,
    /// ERR_NOMOTD = "422"
    ErrNomotd,
    /// ERR_NOADMININFO = "423"
    ErrNoadmininfo,
    /// ERR_FILEERROR = "424"
    ErrFileerror,
    /// ERR_NONICKNAMEGIVEN = "431"
    ErrNonicknamegiven,
    /// ERR_ERRONEOUSNICKNAME = "432"
    ErrErroneousnickname,
    /// ERR_NICKNAMEINUSE = "433"
    ErrNicknameinuse,
    /// ERR_NICKCOLLISION = "436"
    ErrNickcollision,
    /// ERR_UNAVAILRESOURCE = "437"
    ErrUnavailresource,
    /// ERR_USERNOTINCHANNEL = "441"
    ErrUsernotinchannel,
    /// ERR_NOTONCHANNEL = "442"
    ErrNotonchannel,
    /// ERR_USERONCHANNEL = "443"
    ErrUseronchannel,
    /// ERR_NOLOGIN = "444"
    ErrNologin,
    /// ERR_SUMMONDISABLED = "445"
    ErrSummondisabled,
    /// ERR_USERSDISABLED = "446"
    ErrUsersdisabled,
    /// ERR_NOTREGISTERED = "451"
    ErrNotregistered,
    /// ERR_NEEDMOREPARAMS = "461"
    ErrNeedmoreparams,
    /// ERR_ALREADYREGISTRED = "462"
    ErrAlreadyregistred,
    /// ERR_NOPERMFORHOST = "463"
    ErrNopermforhost,
    /// ERR_PASSWDMISMATCH = "464"
    ErrPasswdmismatch,
    /// ERR_YOUREBANNEDCREEP = "465"
    ErrYourebannedcreep,
    /// ERR_YOUWILLBEBANNED = "466"
    ErrYouwillbebanned,
    /// ERR_KEYSET = "467"
    ErrKeyset,
    /// ERR_CHANNELISFULL = "471"
    ErrChannelisfull,
    /// ERR_UNKNOWNMODE = "472"
    ErrUnknownmode,
    /// ERR_INVITEONLYCHAN = "473"
    ErrInviteonlychan,
    /// ERR_BANNEDFROMCHAN = "474"
    ErrBannedfromchan,
    /// ERR_BADCHANNELKEY = "475"
    ErrBadchannelkey,
    /// ERR_BADCHANMASK = "476"
    ErrBadchanmask,
    /// ERR_NOCHANMODES = "477"
    ErrNochanmodes,
    /// ERR_BANLISTFULL = "478"
    ErrBanlistfull,
    /// ERR_NOPRIVILEGES = "481"
    ErrNoprivileges,
    /// ERR_CHANOPRIVSNEEDED = "482"
    ErrChanoprivsneeded,
    /// ERR_CANTKILLSERVER = "483"
    ErrCantkillserver,
    /// ERR_RESTRICTED = "484"
    ErrRestricted,
    /// ERR_UNIQOPPRIVSNEEDED = "485"
    ErrUniqopprivsneeded,
    /// ERR_NOOPERHOST = "491"
    ErrNooperhost,
    /// ERR_UMODEUNKNOWNFLAG = "501"
    ErrUmodeunknownflag,
    /// ERR_USERSDONTMATCH = "502"
    ErrUsersdontmatch,
    /// Codes that are unknown end up in here.
    Unknown(String),
}

impl Code {

    /// Checks if the code is a reply.
    pub fn is_reply(&self) -> bool {
        match *self {
            Code::RplWelcome => true,
            Code::RplYourhost => true,
            Code::RplCreated => true,
            Code::RplMyinfo => true,
            Code::RplBounce => true,
            Code::RplUserhost => true,
            Code::RplIson => true,
            Code::RplAway => true,
            Code::RplUnaway => true,
            Code::RplNowaway => true,
            Code::RplWhoisuser => true,
            Code::RplWhoisserver => true,
            Code::RplWhoisoperator => true,
            Code::RplWhoisidle => true,
            Code::RplEndofwhois => true,
            Code::RplWhoischannels => true,
            Code::RplWhowasuser => true,
            Code::RplEndofwhowas => true,
            Code::RplListstart => true,
            Code::RplList => true,
            Code::RplListend => true,
            Code::RplUniqopis => true,
            Code::RplChannelmodeis => true,
            Code::RplNotopic => true,
            Code::RplTopic => true,
            Code::RplInviting => true,
            Code::RplSummoning => true,
            Code::RplInvitelist => true,
            Code::RplEndofinvitelist => true,
            Code::RplExceptlist => true,
            Code::RplEndofexecptlist => true,
            Code::RplVersion => true,
            Code::RplWhoreply => true,
            Code::RplEndofwho => true,
            Code::RplNamreply => true,
            Code::RplEndofnames => true,
            Code::RplLinks => true,
            Code::RplEndoflinks => true,
            Code::RplBanlist => true,
            Code::RplEndofbanlist => true,
            Code::RplInfo => true,
            Code::RplEndofinfo => true,
            Code::RplMotdstart => true,
            Code::RplMotd => true,
            Code::RplEndofmotd => true,
            Code::RplYoureoper => true,
            Code::RplRehashing => true,
            Code::RplYoureservice => true,
            Code::RplTime => true,
            Code::RplUsersstart => true,
            Code::RplUsers => true,
            Code::RplEndofusers => true,
            Code::RplNousers => true,
            Code::RplTracelink => true,
            Code::RplTraceconnecting => true,
            Code::RplTracehandshake => true,
            Code::RplTraceuknown => true,
            Code::RplTraceoperator => true,
            Code::RplTraceuser => true,
            Code::RplTraceserver => true,
            Code::RplTraceservice => true,
            Code::RplTracenewtype => true,
            Code::RplTraceclass => true,
            Code::RplTracereconnect => true,
            Code::RplTracelog => true,
            Code::RplTraceend => true,
            Code::RplStatslinkinfo => true,
            Code::RplStatscommands => true,
            Code::RplEndofstats => true,
            Code::RplStatsuptime => true,
            Code::RplStatsoline => true,
            Code::RplUmodeis => true,
            Code::RplServlist => true,
            Code::RplServlistend => true,
            Code::RplLuserclient => true,
            Code::RplLuserop => true,
            Code::RplLuserunknown => true,
            Code::RplLuserchannels => true,
            Code::RplLuserme => true,
            Code::RplAdminme => true,
            Code::RplAdminloc1 => true,
            Code::RplAdminloc2 => true,
            Code::RplAdminemail => true,
            Code::RplTryagain => true,
            _  => false,
        }
    }

    /// Check if the code is en error.
    pub fn is_error(&self) -> bool {
        match *self {
            Code::ErrNosuchnick => true,
            Code::ErrNosuchserver => true,
            Code::ErrNosuchchannel => true,
            Code::ErrCannotsendtochan => true,
            Code::ErrToomanychannels => true,
            Code::ErrWasnosuchnick => true,
            Code::ErrToomanytargets => true,
            Code::ErrNosuchservice => true,
            Code::ErrNoorigin => true,
            Code::ErrNorecipient => true,
            Code::ErrNotexttosend => true,
            Code::ErrNotoplevel => true,
            Code::ErrWildtoplevel => true,
            Code::ErrBadmask => true,
            Code::ErrUnknowncommand => true,
            Code::ErrNomotd => true,
            Code::ErrNoadmininfo => true,
            Code::ErrFileerror => true,
            Code::ErrNonicknamegiven => true,
            Code::ErrErroneousnickname => true,
            Code::ErrNicknameinuse => true,
            Code::ErrNickcollision => true,
            Code::ErrUnavailresource => true,
            Code::ErrUsernotinchannel => true,
            Code::ErrNotonchannel => true,
            Code::ErrUseronchannel => true,
            Code::ErrNologin => true,
            Code::ErrSummondisabled => true,
            Code::ErrUsersdisabled => true,
            Code::ErrNotregistered => true,
            Code::ErrNeedmoreparams => true,
            Code::ErrAlreadyregistred => true,
            Code::ErrNopermforhost => true,
            Code::ErrPasswdmismatch => true,
            Code::ErrYourebannedcreep => true,
            Code::ErrYouwillbebanned => true,
            Code::ErrKeyset => true,
            Code::ErrChannelisfull => true,
            Code::ErrUnknownmode => true,
            Code::ErrInviteonlychan => true,
            Code::ErrBannedfromchan => true,
            Code::ErrBadchannelkey => true,
            Code::ErrBadchanmask => true,
            Code::ErrNochanmodes => true,
            Code::ErrBanlistfull => true,
            Code::ErrNoprivileges => true,
            Code::ErrChanoprivsneeded => true,
            Code::ErrCantkillserver => true,
            Code::ErrRestricted => true,
            Code::ErrUniqopprivsneeded => true,
            Code::ErrNooperhost => true,
            Code::ErrUmodeunknownflag => true,
            Code::ErrUsersdontmatch => true,
            _  => false,
        }
    }

}

impl fmt::Display for Code {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            Code::Pass => "PASS",
            Code::Nick => "NICK",
            Code::User => "USER",
            Code::Oper => "OPER",
            Code::Mode => "MODE",
            Code::Service => "SERVICE",
            Code::Quit => "QUIT",
            Code::Squit => "SQUIT",
            Code::Join => "JOIN",
            Code::Part => "PART",
            Code::Topic => "TOPIC",
            Code::Names => "NAMES",
            Code::List => "LIST",
            Code::Invite => "INVITE",
            Code::Kick => "KICK",
            Code::Privmsg => "PRIVMSG",
            Code::Notice => "NOTICE",
            Code::Motd => "MOTD",
            Code::Lusers => "LUSERS",
            Code::Version => "VERSION",
            Code::Stats => "STATS",
            Code::Links => "LINKS",
            Code::Time => "TIME",
            Code::Connect => "CONNECT",
            Code::Trace => "TRACE",
            Code::Admin => "ADMIN",
            Code::Info => "INFO",
            Code::Servlist => "SERVLIST",
            Code::Squery => "SQUERY",
            Code::Who => "WHO",
            Code::Whois => "WHOIS",
            Code::Whowas => "WHOWAS",
            Code::Kill => "KILL",
            Code::Ping => "PING",
            Code::Pong => "PONG",
            Code::Error => "ERROR",
            Code::Away => "AWAY",
            Code::Rehash => "REHASH",
            Code::Die => "DIE",
            Code::Restart => "RESTART",
            Code::Summon => "SUMMON",
            Code::Users => "USERS",
            Code::Wallops => "WALLOPS",
            Code::Userhost => "USERHOST",
            Code::Ison => "ISON",
            Code::RplWelcome => "001",
            Code::RplYourhost => "002",
            Code::RplCreated => "003",
            Code::RplMyinfo => "004",
            Code::RplBounce => "005",
            Code::RplUserhost => "302",
            Code::RplIson => "303",
            Code::RplAway => "301",
            Code::RplUnaway => "305",
            Code::RplNowaway => "306",
            Code::RplWhoisuser => "311",
            Code::RplWhoisserver => "312",
            Code::RplWhoisoperator => "313",
            Code::RplWhoisidle => "317",
            Code::RplEndofwhois => "318",
            Code::RplWhoischannels => "319",
            Code::RplWhowasuser => "314",
            Code::RplEndofwhowas => "369",
            Code::RplListstart => "321",
            Code::RplList => "322",
            Code::RplListend => "323",
            Code::RplUniqopis => "325",
            Code::RplChannelmodeis => "324",
            Code::RplNotopic => "331",
            Code::RplTopic => "332",
            Code::RplInviting => "341",
            Code::RplSummoning => "342",
            Code::RplInvitelist => "346",
            Code::RplEndofinvitelist => "347",
            Code::RplExceptlist => "348",
            Code::RplEndofexecptlist => "349",
            Code::RplVersion => "351",
            Code::RplWhoreply => "352",
            Code::RplEndofwho => "315",
            Code::RplNamreply => "353",
            Code::RplEndofnames => "366",
            Code::RplLinks => "364",
            Code::RplEndoflinks => "365",
            Code::RplBanlist => "367",
            Code::RplEndofbanlist => "368",
            Code::RplInfo => "371",
            Code::RplEndofinfo => "374",
            Code::RplMotdstart => "375",
            Code::RplMotd => "372",
            Code::RplEndofmotd => "376",
            Code::RplYoureoper => "381",
            Code::RplRehashing => "382",
            Code::RplYoureservice => "383",
            Code::RplTime => "391",
            Code::RplUsersstart => "392",
            Code::RplUsers => "393",
            Code::RplEndofusers => "394",
            Code::RplNousers => "395",
            Code::RplTracelink => "200",
            Code::RplTraceconnecting => "201",
            Code::RplTracehandshake => "202",
            Code::RplTraceuknown => "203",
            Code::RplTraceoperator => "204",
            Code::RplTraceuser => "205",
            Code::RplTraceserver => "206",
            Code::RplTraceservice => "207",
            Code::RplTracenewtype => "208",
            Code::RplTraceclass => "209",
            Code::RplTracereconnect => "210",
            Code::RplTracelog => "261",
            Code::RplTraceend => "262",
            Code::RplStatslinkinfo => "211",
            Code::RplStatscommands => "212",
            Code::RplEndofstats => "219",
            Code::RplStatsuptime => "242",
            Code::RplStatsoline => "243",
            Code::RplUmodeis => "221",
            Code::RplServlist => "234",
            Code::RplServlistend => "235",
            Code::RplLuserclient => "251",
            Code::RplLuserop => "252",
            Code::RplLuserunknown => "253",
            Code::RplLuserchannels => "254",
            Code::RplLuserme => "255",
            Code::RplAdminme => "256",
            Code::RplAdminloc1 => "257",
            Code::RplAdminloc2 => "258",
            Code::RplAdminemail => "259",
            Code::RplTryagain => "263",
            Code::ErrNosuchnick => "401",
            Code::ErrNosuchserver => "402",
            Code::ErrNosuchchannel => "403",
            Code::ErrCannotsendtochan => "404",
            Code::ErrToomanychannels => "405",
            Code::ErrWasnosuchnick => "406",
            Code::ErrToomanytargets => "407",
            Code::ErrNosuchservice => "408",
            Code::ErrNoorigin => "409",
            Code::ErrNorecipient => "411",
            Code::ErrNotexttosend => "412",
            Code::ErrNotoplevel => "413",
            Code::ErrWildtoplevel => "414",
            Code::ErrBadmask => "415",
            Code::ErrUnknowncommand => "421",
            Code::ErrNomotd => "422",
            Code::ErrNoadmininfo => "423",
            Code::ErrFileerror => "424",
            Code::ErrNonicknamegiven => "431",
            Code::ErrErroneousnickname => "432",
            Code::ErrNicknameinuse => "433",
            Code::ErrNickcollision => "436",
            Code::ErrUnavailresource => "437",
            Code::ErrUsernotinchannel => "441",
            Code::ErrNotonchannel => "442",
            Code::ErrUseronchannel => "443",
            Code::ErrNologin => "444",
            Code::ErrSummondisabled => "445",
            Code::ErrUsersdisabled => "446",
            Code::ErrNotregistered => "451",
            Code::ErrNeedmoreparams => "461",
            Code::ErrAlreadyregistred => "462",
            Code::ErrNopermforhost => "463",
            Code::ErrPasswdmismatch => "464",
            Code::ErrYourebannedcreep => "465",
            Code::ErrYouwillbebanned => "466",
            Code::ErrKeyset => "467",
            Code::ErrChannelisfull => "471",
            Code::ErrUnknownmode => "472",
            Code::ErrInviteonlychan => "473",
            Code::ErrBannedfromchan => "474",
            Code::ErrBadchannelkey => "475",
            Code::ErrBadchanmask => "476",
            Code::ErrNochanmodes => "477",
            Code::ErrBanlistfull => "478",
            Code::ErrNoprivileges => "481",
            Code::ErrChanoprivsneeded => "482",
            Code::ErrCantkillserver => "483",
            Code::ErrRestricted => "484",
            Code::ErrUniqopprivsneeded => "485",
            Code::ErrNooperhost => "491",
            Code::ErrUmodeunknownflag => "501",
            Code::ErrUsersdontmatch => "502",
            Code::Unknown(ref text) => &text[..],
        };
        f.write_str(text)
    }

}

impl str::FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Code, ()> {
        let code = match s {
            "PASS" => Code::Pass,
            "NICK" => Code::Nick,
            "USER" => Code::User,
            "OPER" => Code::Oper,
            "MODE" => Code::Mode,
            "SERVICE" => Code::Service,
            "QUIT" => Code::Quit,
            "SQUIT" => Code::Squit,
            "JOIN" => Code::Join,
            "PART" => Code::Part,
            "TOPIC" => Code::Topic,
            "NAMES" => Code::Names,
            "LIST" => Code::List,
            "INVITE" => Code::Invite,
            "KICK" => Code::Kick,
            "PRIVMSG" => Code::Privmsg,
            "NOTICE" => Code::Notice,
            "MOTD" => Code::Motd,
            "LUSERS" => Code::Lusers,
            "VERSION" => Code::Version,
            "STATS" => Code::Stats,
            "LINKS" => Code::Links,
            "TIME" => Code::Time,
            "CONNECT" => Code::Connect,
            "TRACE" => Code::Trace,
            "ADMIN" => Code::Admin,
            "INFO" => Code::Info,
            "SERVLIST" => Code::Servlist,
            "SQUERY" => Code::Squery,
            "WHO" => Code::Who,
            "WHOIS" => Code::Whois,
            "WHOWAS" => Code::Whowas,
            "KILL" => Code::Kill,
            "PING" => Code::Ping,
            "PONG" => Code::Pong,
            "ERROR" => Code::Error,
            "AWAY" => Code::Away,
            "REHASH" => Code::Rehash,
            "DIE" => Code::Die,
            "RESTART" => Code::Restart,
            "SUMMON" => Code::Summon,
            "USERS" => Code::Users,
            "WALLOPS" => Code::Wallops,
            "USERHOST" => Code::Userhost,
            "ISON" => Code::Ison,
            "001" => Code::RplWelcome,
            "002" => Code::RplYourhost,
            "003" => Code::RplCreated,
            "004" => Code::RplMyinfo,
            "005" => Code::RplBounce,
            "302" => Code::RplUserhost,
            "303" => Code::RplIson,
            "301" => Code::RplAway,
            "305" => Code::RplUnaway,
            "306" => Code::RplNowaway,
            "311" => Code::RplWhoisuser,
            "312" => Code::RplWhoisserver,
            "313" => Code::RplWhoisoperator,
            "317" => Code::RplWhoisidle,
            "318" => Code::RplEndofwhois,
            "319" => Code::RplWhoischannels,
            "314" => Code::RplWhowasuser,
            "369" => Code::RplEndofwhowas,
            "321" => Code::RplListstart,
            "322" => Code::RplList,
            "323" => Code::RplListend,
            "325" => Code::RplUniqopis,
            "324" => Code::RplChannelmodeis,
            "331" => Code::RplNotopic,
            "332" => Code::RplTopic,
            "341" => Code::RplInviting,
            "342" => Code::RplSummoning,
            "346" => Code::RplInvitelist,
            "347" => Code::RplEndofinvitelist,
            "348" => Code::RplExceptlist,
            "349" => Code::RplEndofexecptlist,
            "351" => Code::RplVersion,
            "352" => Code::RplWhoreply,
            "315" => Code::RplEndofwho,
            "353" => Code::RplNamreply,
            "366" => Code::RplEndofnames,
            "364" => Code::RplLinks,
            "365" => Code::RplEndoflinks,
            "367" => Code::RplBanlist,
            "368" => Code::RplEndofbanlist,
            "371" => Code::RplInfo,
            "374" => Code::RplEndofinfo,
            "375" => Code::RplMotdstart,
            "372" => Code::RplMotd,
            "376" => Code::RplEndofmotd,
            "381" => Code::RplYoureoper,
            "382" => Code::RplRehashing,
            "383" => Code::RplYoureservice,
            "391" => Code::RplTime,
            "392" => Code::RplUsersstart,
            "393" => Code::RplUsers,
            "394" => Code::RplEndofusers,
            "395" => Code::RplNousers,
            "200" => Code::RplTracelink,
            "201" => Code::RplTraceconnecting,
            "202" => Code::RplTracehandshake,
            "203" => Code::RplTraceuknown,
            "204" => Code::RplTraceoperator,
            "205" => Code::RplTraceuser,
            "206" => Code::RplTraceserver,
            "207" => Code::RplTraceservice,
            "208" => Code::RplTracenewtype,
            "209" => Code::RplTraceclass,
            "210" => Code::RplTracereconnect,
            "261" => Code::RplTracelog,
            "262" => Code::RplTraceend,
            "211" => Code::RplStatslinkinfo,
            "212" => Code::RplStatscommands,
            "219" => Code::RplEndofstats,
            "242" => Code::RplStatsuptime,
            "243" => Code::RplStatsoline,
            "221" => Code::RplUmodeis,
            "234" => Code::RplServlist,
            "235" => Code::RplServlistend,
            "251" => Code::RplLuserclient,
            "252" => Code::RplLuserop,
            "253" => Code::RplLuserunknown,
            "254" => Code::RplLuserchannels,
            "255" => Code::RplLuserme,
            "256" => Code::RplAdminme,
            "257" => Code::RplAdminloc1,
            "258" => Code::RplAdminloc2,
            "259" => Code::RplAdminemail,
            "263" => Code::RplTryagain,
            "401" => Code::ErrNosuchnick,
            "402" => Code::ErrNosuchserver,
            "403" => Code::ErrNosuchchannel,
            "404" => Code::ErrCannotsendtochan,
            "405" => Code::ErrToomanychannels,
            "406" => Code::ErrWasnosuchnick,
            "407" => Code::ErrToomanytargets,
            "408" => Code::ErrNosuchservice,
            "409" => Code::ErrNoorigin,
            "411" => Code::ErrNorecipient,
            "412" => Code::ErrNotexttosend,
            "413" => Code::ErrNotoplevel,
            "414" => Code::ErrWildtoplevel,
            "415" => Code::ErrBadmask,
            "421" => Code::ErrUnknowncommand,
            "422" => Code::ErrNomotd,
            "423" => Code::ErrNoadmininfo,
            "424" => Code::ErrFileerror,
            "431" => Code::ErrNonicknamegiven,
            "432" => Code::ErrErroneousnickname,
            "433" => Code::ErrNicknameinuse,
            "436" => Code::ErrNickcollision,
            "437" => Code::ErrUnavailresource,
            "441" => Code::ErrUsernotinchannel,
            "442" => Code::ErrNotonchannel,
            "443" => Code::ErrUseronchannel,
            "444" => Code::ErrNologin,
            "445" => Code::ErrSummondisabled,
            "446" => Code::ErrUsersdisabled,
            "451" => Code::ErrNotregistered,
            "461" => Code::ErrNeedmoreparams,
            "462" => Code::ErrAlreadyregistred,
            "463" => Code::ErrNopermforhost,
            "464" => Code::ErrPasswdmismatch,
            "465" => Code::ErrYourebannedcreep,
            "466" => Code::ErrYouwillbebanned,
            "467" => Code::ErrKeyset,
            "471" => Code::ErrChannelisfull,
            "472" => Code::ErrUnknownmode,
            "473" => Code::ErrInviteonlychan,
            "474" => Code::ErrBannedfromchan,
            "475" => Code::ErrBadchannelkey,
            "476" => Code::ErrBadchanmask,
            "477" => Code::ErrNochanmodes,
            "478" => Code::ErrBanlistfull,
            "481" => Code::ErrNoprivileges,
            "482" => Code::ErrChanoprivsneeded,
            "483" => Code::ErrCantkillserver,
            "484" => Code::ErrRestricted,
            "485" => Code::ErrUniqopprivsneeded,
            "491" => Code::ErrNooperhost,
            "501" => Code::ErrUmodeunknownflag,
            "502" => Code::ErrUsersdontmatch,
            _ => Code::Unknown(s.to_string()),
        };
        Ok(code)
    }
}
