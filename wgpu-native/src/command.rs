/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::GLOBAL;

use core::{gfx_select, id};


#[no_mangle]
pub extern "C" fn wgpu_command_encoder_finish(
    encoder_id: id::CommandEncoderId,
    desc: Option<&core::command::CommandBufferDescriptor>,
) -> id::CommandBufferId {
    let desc = &desc.cloned().unwrap_or_default();
    gfx_select!(encoder_id => GLOBAL.command_encoder_finish(encoder_id, desc))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_buffer_to_buffer(
    command_encoder_id: id::CommandEncoderId,
    source: id::BufferId,
    source_offset: core::BufferAddress,
    destination: id::BufferId,
    destination_offset: core::BufferAddress,
    size: core::BufferAddress,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_buffer_to_buffer(
        command_encoder_id,
        source, source_offset,
        destination,
        destination_offset,
        size))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_buffer_to_texture(
    command_encoder_id: id::CommandEncoderId,
    source: &core::command::BufferCopyView,
    destination: &core::command::TextureCopyView,
    copy_size: core::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_buffer_to_texture(
        command_encoder_id,
        source,
        destination,
        copy_size))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_texture_to_buffer(
    command_encoder_id: id::CommandEncoderId,
    source: &core::command::TextureCopyView,
    destination: &core::command::BufferCopyView,
    copy_size: core::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_texture_to_buffer(
        command_encoder_id,
        source,
        destination,
        copy_size))
}

#[no_mangle]
pub extern "C" fn wgpu_command_encoder_copy_texture_to_texture(
    command_encoder_id: id::CommandEncoderId,
    source: &core::command::TextureCopyView,
    destination: &core::command::TextureCopyView,
    copy_size: core::Extent3d,
) {
    gfx_select!(command_encoder_id => GLOBAL.command_encoder_copy_texture_to_texture(
        command_encoder_id,
        source,
        destination,
        copy_size))
}


#[no_mangle]
pub unsafe extern "C" fn wgpu_render_pass_end_pass(pass_id: id::RenderPassId) {
    let (pass_data, encoder_id, targets) = Box::from_raw(pass_id).finish_render();
    let color_attachments: arrayvec::ArrayVec<[_; core::device::MAX_COLOR_TARGETS]> = targets.colors
        .iter()
        .flat_map(|at| {
            if at.attachment == id::TextureViewId::ERROR {
                None
            } else {
                Some(core::command::RenderPassColorAttachmentDescriptor {
                    attachment: at.attachment,
                    resolve_target: if at.resolve_target == id::TextureViewId::ERROR {
                        None
                    } else {
                        Some(&at.resolve_target)
                    },
                    load_op: at.load_op,
                    store_op: at.store_op,
                    clear_color: at.clear_color,
                })
            }
        })
        .collect();
    let depth_stencil_attachment = if targets.depth_stencil.attachment == id::TextureViewId::ERROR {
        None
    } else {
        Some(&targets.depth_stencil)
    };
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_render_pass(encoder_id, &color_attachments, depth_stencil_attachment, &pass_data))
}

#[no_mangle]
pub unsafe extern "C" fn wgpu_compute_pass_end_pass(pass_id: id::ComputePassId) {
    let (pass_data, encoder_id) = Box::from_raw(pass_id).finish_compute();
    gfx_select!(encoder_id => GLOBAL.command_encoder_run_compute_pass(encoder_id, &pass_data))
}
