use crate::user_service::use_cases::UserService;

enum RequestContent {}

struct RequestCommand {
    user_service: UserService,
    command_content: RequestContent,
}
