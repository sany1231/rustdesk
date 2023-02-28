use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_start_global_event_stream(port_: i64, app_type: *mut wire_uint_8_list) {
    wire_start_global_event_stream_impl(port_, app_type)
}

#[no_mangle]
pub extern "C" fn wire_stop_global_event_stream(port_: i64, app_type: *mut wire_uint_8_list) {
    wire_stop_global_event_stream_impl(port_, app_type)
}

#[no_mangle]
pub extern "C" fn wire_host_stop_system_key_propagate(port_: i64, _stopped: bool) {
    wire_host_stop_system_key_propagate_impl(port_, _stopped)
}

#[no_mangle]
pub extern "C" fn wire_session_add_sync(
    id: *mut wire_uint_8_list,
    is_file_transfer: bool,
    is_port_forward: bool,
) -> support::WireSyncReturn {
    wire_session_add_sync_impl(id, is_file_transfer, is_port_forward)
}

#[no_mangle]
pub extern "C" fn wire_session_start(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_start_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_get_remember(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_get_remember_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_get_toggle_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    arg: *mut wire_uint_8_list,
) {
    wire_session_get_toggle_option_impl(port_, id, arg)
}

#[no_mangle]
pub extern "C" fn wire_session_get_toggle_option_sync(
    id: *mut wire_uint_8_list,
    arg: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_toggle_option_sync_impl(id, arg)
}

#[no_mangle]
pub extern "C" fn wire_session_get_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    arg: *mut wire_uint_8_list,
) {
    wire_session_get_option_impl(port_, id, arg)
}

#[no_mangle]
pub extern "C" fn wire_session_login(
    port_: i64,
    id: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
    remember: bool,
) {
    wire_session_login_impl(port_, id, password, remember)
}

#[no_mangle]
pub extern "C" fn wire_session_close(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_close_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_refresh(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_refresh_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_record_screen(
    port_: i64,
    id: *mut wire_uint_8_list,
    start: bool,
    width: usize,
    height: usize,
) {
    wire_session_record_screen_impl(port_, id, start, width, height)
}

#[no_mangle]
pub extern "C" fn wire_session_reconnect(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_reconnect_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_toggle_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_toggle_option_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_flutter_config(
    port_: i64,
    id: *mut wire_uint_8_list,
    k: *mut wire_uint_8_list,
) {
    wire_session_get_flutter_config_impl(port_, id, k)
}

#[no_mangle]
pub extern "C" fn wire_session_set_flutter_config(
    port_: i64,
    id: *mut wire_uint_8_list,
    k: *mut wire_uint_8_list,
    v: *mut wire_uint_8_list,
) {
    wire_session_set_flutter_config_impl(port_, id, k, v)
}

#[no_mangle]
pub extern "C" fn wire_get_local_flutter_config(
    k: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_get_local_flutter_config_impl(k)
}

#[no_mangle]
pub extern "C" fn wire_set_local_flutter_config(
    port_: i64,
    k: *mut wire_uint_8_list,
    v: *mut wire_uint_8_list,
) {
    wire_set_local_flutter_config_impl(port_, k, v)
}

#[no_mangle]
pub extern "C" fn wire_get_local_kb_layout_type() -> support::WireSyncReturn {
    wire_get_local_kb_layout_type_impl()
}

#[no_mangle]
pub extern "C" fn wire_set_local_kb_layout_type(port_: i64, kb_layout_type: *mut wire_uint_8_list) {
    wire_set_local_kb_layout_type_impl(port_, kb_layout_type)
}

#[no_mangle]
pub extern "C" fn wire_session_get_view_style(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_get_view_style_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_view_style(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_view_style_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_scroll_style(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_get_scroll_style_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_scroll_style(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_scroll_style_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_image_quality(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_get_image_quality_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_image_quality(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_image_quality_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_keyboard_mode(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_get_keyboard_mode_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_keyboard_mode(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_keyboard_mode_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_custom_image_quality(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_get_custom_image_quality_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_is_keyboard_mode_supported(
    id: *mut wire_uint_8_list,
    mode: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_is_keyboard_mode_supported_impl(id, mode)
}

#[no_mangle]
pub extern "C" fn wire_session_set_custom_image_quality(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: i32,
) {
    wire_session_set_custom_image_quality_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_set_custom_fps(port_: i64, id: *mut wire_uint_8_list, fps: i32) {
    wire_session_set_custom_fps_impl(port_, id, fps)
}

#[no_mangle]
pub extern "C" fn wire_session_lock_screen(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_lock_screen_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_ctrl_alt_del(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_ctrl_alt_del_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_switch_display(port_: i64, id: *mut wire_uint_8_list, value: i32) {
    wire_session_switch_display_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_handle_flutter_key_event(
    port_: i64,
    id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
    keycode: i32,
    scancode: i32,
    lock_modes: i32,
    down_or_up: bool,
) {
    wire_session_handle_flutter_key_event_impl(
        port_, id, name, keycode, scancode, lock_modes, down_or_up,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_enter_or_leave(port_: i64, id: *mut wire_uint_8_list, enter: bool) {
    wire_session_enter_or_leave_impl(port_, id, enter)
}

#[no_mangle]
pub extern "C" fn wire_session_input_key(
    port_: i64,
    id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
    down: bool,
    press: bool,
    alt: bool,
    ctrl: bool,
    shift: bool,
    command: bool,
) {
    wire_session_input_key_impl(port_, id, name, down, press, alt, ctrl, shift, command)
}

#[no_mangle]
pub extern "C" fn wire_session_input_string(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_input_string_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_send_chat(
    port_: i64,
    id: *mut wire_uint_8_list,
    text: *mut wire_uint_8_list,
) {
    wire_session_send_chat_impl(port_, id, text)
}

#[no_mangle]
pub extern "C" fn wire_session_peer_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_peer_option_impl(port_, id, name, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_peer_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
) {
    wire_session_get_peer_option_impl(port_, id, name)
}

#[no_mangle]
pub extern "C" fn wire_session_input_os_password(
    port_: i64,
    id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_input_os_password_impl(port_, id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_read_remote_dir(
    port_: i64,
    id: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
    include_hidden: bool,
) {
    wire_session_read_remote_dir_impl(port_, id, path, include_hidden)
}

#[no_mangle]
pub extern "C" fn wire_session_send_files(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    to: *mut wire_uint_8_list,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
) {
    wire_session_send_files_impl(
        port_,
        id,
        act_id,
        path,
        to,
        file_num,
        include_hidden,
        is_remote,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_set_confirm_override_file(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    file_num: i32,
    need_override: bool,
    remember: bool,
    is_upload: bool,
) {
    wire_session_set_confirm_override_file_impl(
        port_,
        id,
        act_id,
        file_num,
        need_override,
        remember,
        is_upload,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_remove_file(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    file_num: i32,
    is_remote: bool,
) {
    wire_session_remove_file_impl(port_, id, act_id, path, file_num, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_read_dir_recursive(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    is_remote: bool,
    show_hidden: bool,
) {
    wire_session_read_dir_recursive_impl(port_, id, act_id, path, is_remote, show_hidden)
}

#[no_mangle]
pub extern "C" fn wire_session_remove_all_empty_dirs(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    is_remote: bool,
) {
    wire_session_remove_all_empty_dirs_impl(port_, id, act_id, path, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_cancel_job(port_: i64, id: *mut wire_uint_8_list, act_id: i32) {
    wire_session_cancel_job_impl(port_, id, act_id)
}

#[no_mangle]
pub extern "C" fn wire_session_create_dir(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    is_remote: bool,
) {
    wire_session_create_dir_impl(port_, id, act_id, path, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_read_local_dir_sync(
    port_: i64,
    _id: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
    show_hidden: bool,
) {
    wire_session_read_local_dir_sync_impl(port_, _id, path, show_hidden)
}

#[no_mangle]
pub extern "C" fn wire_session_get_platform(
    port_: i64,
    id: *mut wire_uint_8_list,
    is_remote: bool,
) {
    wire_session_get_platform_impl(port_, id, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_load_last_transfer_jobs(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_load_last_transfer_jobs_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_add_job(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    to: *mut wire_uint_8_list,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
) {
    wire_session_add_job_impl(
        port_,
        id,
        act_id,
        path,
        to,
        file_num,
        include_hidden,
        is_remote,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_resume_job(
    port_: i64,
    id: *mut wire_uint_8_list,
    act_id: i32,
    is_remote: bool,
) {
    wire_session_resume_job_impl(port_, id, act_id, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_elevate_direct(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_elevate_direct_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_elevate_with_logon(
    port_: i64,
    id: *mut wire_uint_8_list,
    username: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
) {
    wire_session_elevate_with_logon_impl(port_, id, username, password)
}

#[no_mangle]
pub extern "C" fn wire_main_get_sound_inputs(port_: i64) {
    wire_main_get_sound_inputs_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_change_id(port_: i64, new_id: *mut wire_uint_8_list) {
    wire_main_change_id_impl(port_, new_id)
}

#[no_mangle]
pub extern "C" fn wire_main_get_async_status(port_: i64) {
    wire_main_get_async_status_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_option(port_: i64, key: *mut wire_uint_8_list) {
    wire_main_get_option_impl(port_, key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_error(port_: i64) {
    wire_main_get_error_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_set_option(
    port_: i64,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_option_impl(port_, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_get_options(port_: i64) {
    wire_main_get_options_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_set_options(port_: i64, json: *mut wire_uint_8_list) {
    wire_main_set_options_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_main_test_if_valid_server(port_: i64, server: *mut wire_uint_8_list) {
    wire_main_test_if_valid_server_impl(port_, server)
}

#[no_mangle]
pub extern "C" fn wire_main_set_socks(
    port_: i64,
    proxy: *mut wire_uint_8_list,
    username: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
) {
    wire_main_set_socks_impl(port_, proxy, username, password)
}

#[no_mangle]
pub extern "C" fn wire_main_get_socks(port_: i64) {
    wire_main_get_socks_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_app_name(port_: i64) {
    wire_main_get_app_name_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_app_name_sync() -> support::WireSyncReturn {
    wire_main_get_app_name_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_get_license(port_: i64) {
    wire_main_get_license_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_version(port_: i64) {
    wire_main_get_version_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_fav(port_: i64) {
    wire_main_get_fav_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_store_fav(port_: i64, favs: *mut wire_StringList) {
    wire_main_store_fav_impl(port_, favs)
}

#[no_mangle]
pub extern "C" fn wire_main_get_peer(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_get_peer_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_get_lan_peers(port_: i64) {
    wire_main_get_lan_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_connect_status(port_: i64) {
    wire_main_get_connect_status_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_check_connect_status(port_: i64) {
    wire_main_check_connect_status_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_is_using_public_server(port_: i64) {
    wire_main_is_using_public_server_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_discover(port_: i64) {
    wire_main_discover_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_api_server(port_: i64) {
    wire_main_get_api_server_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_post_request(
    port_: i64,
    url: *mut wire_uint_8_list,
    body: *mut wire_uint_8_list,
    header: *mut wire_uint_8_list,
) {
    wire_main_post_request_impl(port_, url, body, header)
}

#[no_mangle]
pub extern "C" fn wire_main_get_local_option(
    key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_get_local_option_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_set_local_option(
    port_: i64,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_local_option_impl(port_, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_get_my_id(port_: i64) {
    wire_main_get_my_id_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_uuid(port_: i64) {
    wire_main_get_uuid_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_peer_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
) {
    wire_main_get_peer_option_impl(port_, id, key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_peer_option_sync(
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_get_peer_option_sync_impl(id, key)
}

#[no_mangle]
pub extern "C" fn wire_main_set_peer_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_peer_option_impl(port_, id, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_set_peer_option_sync(
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_set_peer_option_sync_impl(id, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_set_peer_alias(
    port_: i64,
    id: *mut wire_uint_8_list,
    alias: *mut wire_uint_8_list,
) {
    wire_main_set_peer_alias_impl(port_, id, alias)
}

#[no_mangle]
pub extern "C" fn wire_main_forget_password(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_forget_password_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_peer_has_password(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_peer_has_password_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_load_recent_peers(port_: i64) {
    wire_main_load_recent_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_load_fav_peers(port_: i64) {
    wire_main_load_fav_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_load_lan_peers(port_: i64) {
    wire_main_load_lan_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_change_theme(port_: i64, dark: *mut wire_uint_8_list) {
    wire_main_change_theme_impl(port_, dark)
}

#[no_mangle]
pub extern "C" fn wire_main_change_language(port_: i64, lang: *mut wire_uint_8_list) {
    wire_main_change_language_impl(port_, lang)
}

#[no_mangle]
pub extern "C" fn wire_main_default_video_save_directory(port_: i64) {
    wire_main_default_video_save_directory_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_session_add_port_forward(
    port_: i64,
    id: *mut wire_uint_8_list,
    local_port: i32,
    remote_host: *mut wire_uint_8_list,
    remote_port: i32,
) {
    wire_session_add_port_forward_impl(port_, id, local_port, remote_host, remote_port)
}

#[no_mangle]
pub extern "C" fn wire_session_remove_port_forward(
    port_: i64,
    id: *mut wire_uint_8_list,
    local_port: i32,
) {
    wire_session_remove_port_forward_impl(port_, id, local_port)
}

#[no_mangle]
pub extern "C" fn wire_session_new_rdp(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_new_rdp_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_get_last_remote_id(port_: i64) {
    wire_main_get_last_remote_id_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_software_update_url(port_: i64) {
    wire_main_get_software_update_url_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_home_dir(port_: i64) {
    wire_main_get_home_dir_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_langs(port_: i64) {
    wire_main_get_langs_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_temporary_password(port_: i64) {
    wire_main_get_temporary_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_permanent_password(port_: i64) {
    wire_main_get_permanent_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_online_statue(port_: i64) {
    wire_main_get_online_statue_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_cm_get_clients_state(port_: i64) {
    wire_cm_get_clients_state_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_cm_check_clients_length(port_: i64, length: usize) {
    wire_cm_check_clients_length_impl(port_, length)
}

#[no_mangle]
pub extern "C" fn wire_cm_get_clients_length(port_: i64) {
    wire_cm_get_clients_length_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_init(port_: i64, app_dir: *mut wire_uint_8_list) {
    wire_main_init_impl(port_, app_dir)
}

#[no_mangle]
pub extern "C" fn wire_main_device_id(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_device_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_device_name(port_: i64, name: *mut wire_uint_8_list) {
    wire_main_device_name_impl(port_, name)
}

#[no_mangle]
pub extern "C" fn wire_main_remove_peer(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_remove_peer_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_has_hwcodec() -> support::WireSyncReturn {
    wire_main_has_hwcodec_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_root(port_: i64) {
    wire_main_is_root_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_double_click_time() -> support::WireSyncReturn {
    wire_get_double_click_time_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_start_dbus_server(port_: i64) {
    wire_main_start_dbus_server_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_session_send_mouse(
    port_: i64,
    id: *mut wire_uint_8_list,
    msg: *mut wire_uint_8_list,
) {
    wire_session_send_mouse_impl(port_, id, msg)
}

#[no_mangle]
pub extern "C" fn wire_session_restart_remote_device(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_restart_remote_device_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_get_audit_server_sync(
    id: *mut wire_uint_8_list,
    typ: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_audit_server_sync_impl(id, typ)
}

#[no_mangle]
pub extern "C" fn wire_session_send_note(
    port_: i64,
    id: *mut wire_uint_8_list,
    note: *mut wire_uint_8_list,
) {
    wire_session_send_note_impl(port_, id, note)
}

#[no_mangle]
pub extern "C" fn wire_session_supported_hwcodec(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_supported_hwcodec_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_session_change_prefer_codec(port_: i64, id: *mut wire_uint_8_list) {
    wire_session_change_prefer_codec_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_set_home_dir(port_: i64, _home: *mut wire_uint_8_list) {
    wire_main_set_home_dir_impl(port_, _home)
}

#[no_mangle]
pub extern "C" fn wire_main_get_data_dir_ios() -> support::WireSyncReturn {
    wire_main_get_data_dir_ios_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_stop_service(port_: i64) {
    wire_main_stop_service_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_start_service(port_: i64) {
    wire_main_start_service_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_update_temporary_password(port_: i64) {
    wire_main_update_temporary_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_set_permanent_password(port_: i64, password: *mut wire_uint_8_list) {
    wire_main_set_permanent_password_impl(port_, password)
}

#[no_mangle]
pub extern "C" fn wire_main_check_super_user_permission(port_: i64) {
    wire_main_check_super_user_permission_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_check_mouse_time(port_: i64) {
    wire_main_check_mouse_time_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_mouse_time(port_: i64) {
    wire_main_get_mouse_time_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_wol(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_wol_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_create_shortcut(port_: i64, _id: *mut wire_uint_8_list) {
    wire_main_create_shortcut_impl(port_, _id)
}

#[no_mangle]
pub extern "C" fn wire_cm_send_chat(port_: i64, conn_id: i32, msg: *mut wire_uint_8_list) {
    wire_cm_send_chat_impl(port_, conn_id, msg)
}

#[no_mangle]
pub extern "C" fn wire_cm_login_res(port_: i64, conn_id: i32, res: bool) {
    wire_cm_login_res_impl(port_, conn_id, res)
}

#[no_mangle]
pub extern "C" fn wire_cm_close_connection(port_: i64, conn_id: i32) {
    wire_cm_close_connection_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_remove_disconnected_connection(port_: i64, conn_id: i32) {
    wire_cm_remove_disconnected_connection_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_check_click_time(port_: i64, conn_id: i32) {
    wire_cm_check_click_time_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_get_click_time(port_: i64) {
    wire_cm_get_click_time_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_cm_switch_permission(
    port_: i64,
    conn_id: i32,
    name: *mut wire_uint_8_list,
    enabled: bool,
) {
    wire_cm_switch_permission_impl(port_, conn_id, name, enabled)
}

#[no_mangle]
pub extern "C" fn wire_cm_can_elevate() -> support::WireSyncReturn {
    wire_cm_can_elevate_impl()
}

#[no_mangle]
pub extern "C" fn wire_cm_elevate_portable(port_: i64, conn_id: i32) {
    wire_cm_elevate_portable_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_main_get_icon(port_: i64) {
    wire_main_get_icon_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_build_date(port_: i64) {
    wire_main_get_build_date_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_query_onlines(port_: i64, ids: *mut wire_StringList) {
    wire_query_onlines_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_version_to_number(port_: i64, v: *mut wire_uint_8_list) {
    wire_version_to_number_impl(port_, v)
}

#[no_mangle]
pub extern "C" fn wire_option_synced(port_: i64) {
    wire_option_synced_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_is_installed() -> support::WireSyncReturn {
    wire_main_is_installed_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_start_grab_keyboard() -> support::WireSyncReturn {
    wire_main_start_grab_keyboard_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_installed_lower_version() -> support::WireSyncReturn {
    wire_main_is_installed_lower_version_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_installed_daemon(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_installed_daemon_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_process_trusted(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_process_trusted_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_can_screen_recording(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_can_screen_recording_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_can_input_monitoring(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_can_input_monitoring_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_share_rdp() -> support::WireSyncReturn {
    wire_main_is_share_rdp_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_rdp_service_open() -> support::WireSyncReturn {
    wire_main_is_rdp_service_open_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_goto_install() -> support::WireSyncReturn {
    wire_main_goto_install_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_get_new_version() -> support::WireSyncReturn {
    wire_main_get_new_version_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_update_me() -> support::WireSyncReturn {
    wire_main_update_me_impl()
}

#[no_mangle]
pub extern "C" fn wire_set_cur_session_id(port_: i64, id: *mut wire_uint_8_list) {
    wire_set_cur_session_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_install_show_run_without_install() -> support::WireSyncReturn {
    wire_install_show_run_without_install_impl()
}

#[no_mangle]
pub extern "C" fn wire_install_run_without_install(port_: i64) {
    wire_install_run_without_install_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_install_install_me(
    port_: i64,
    options: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
) {
    wire_install_install_me_impl(port_, options, path)
}

#[no_mangle]
pub extern "C" fn wire_install_install_path() -> support::WireSyncReturn {
    wire_install_install_path_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_account_auth(port_: i64, op: *mut wire_uint_8_list) {
    wire_main_account_auth_impl(port_, op)
}

#[no_mangle]
pub extern "C" fn wire_main_account_auth_cancel(port_: i64) {
    wire_main_account_auth_cancel_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_account_auth_result(port_: i64) {
    wire_main_account_auth_result_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_on_main_window_close(port_: i64) {
    wire_main_on_main_window_close_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_current_is_wayland() -> support::WireSyncReturn {
    wire_main_current_is_wayland_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_login_wayland() -> support::WireSyncReturn {
    wire_main_is_login_wayland_impl()
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
