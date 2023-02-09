// Copyright 2023 - 2023, Karsten Becker and the rust babeltrace2-plugin contributors
// SPDX-License-Identifier: GPL-2.0-or-later
#include "wrapper.h"

const bt_port *bt_self_component_port_as_port_inline(
    bt_self_component_port *self_component_port) {
  return bt_self_component_port_as_port(self_component_port);
}

const bt_port_input *bt_self_component_port_input_as_port_input_inline(
    const bt_self_component_port_input *self_component_port) {
  return bt_self_component_port_input_as_port_input(self_component_port);
}

const bt_port_output *bt_self_component_port_output_as_port_output_inline(
    bt_self_component_port_output *self_component_port) {
  return bt_self_component_port_output_as_port_output(self_component_port);
}

bt_self_component_port *
bt_self_component_port_input_as_self_component_port_inline(
    bt_self_component_port_input *self_component_port) {
  return bt_self_component_port_input_as_self_component_port(
      self_component_port);
}

bt_self_component_port *
bt_self_component_port_output_as_self_component_port_inline(
    bt_self_component_port_output *self_component_port) {
  return bt_self_component_port_output_as_self_component_port(
      self_component_port);
}

bt_component_class *bt_component_class_source_as_component_class_inline(
    bt_component_class_source *component_class) {
  return bt_component_class_source_as_component_class(component_class);
}

bt_component_class *bt_component_class_filter_as_component_class_inline(
    bt_component_class_filter *component_class) {
  return bt_component_class_filter_as_component_class(component_class);
}

bt_component_class *bt_component_class_sink_as_component_class_inline(
    bt_component_class_sink *component_class) {
  return bt_component_class_sink_as_component_class(component_class);
}

const bt_component *
bt_self_component_as_component_inline(bt_self_component *self_component) {
  return bt_self_component_as_component(self_component);
}

const bt_component_source *bt_self_component_source_as_component_source_inline(
    bt_self_component_source *self_component) {
  return bt_self_component_source_as_component_source(self_component);
}

const bt_component_filter *bt_self_component_filter_as_component_filter_inline(
    bt_self_component_filter *self_component) {
  return bt_self_component_filter_as_component_filter(self_component);
}

const bt_component_sink *bt_self_component_sink_as_component_sink_inline(
    bt_self_component_sink *self_component) {
  return bt_self_component_sink_as_component_sink(self_component);
}

bt_self_component *bt_self_component_source_as_self_component_inline(
    bt_self_component_source *self_component) {
  return bt_self_component_source_as_self_component(self_component);
}

bt_self_component *bt_self_component_filter_as_self_component_inline(
    bt_self_component_filter *self_component) {
  return bt_self_component_filter_as_self_component(self_component);
}

bt_self_component *bt_self_component_sink_as_self_component_inline(
    bt_self_component_sink *self_component) {
  return bt_self_component_sink_as_self_component(self_component);
}

const bt_component_class *bt_self_component_class_as_component_class_inline(
    bt_self_component_class *self_component_class) {
  return bt_self_component_class_as_component_class(self_component_class);
}

const bt_component_class_source *
bt_self_component_class_source_as_component_class_source_inline(
    bt_self_component_class_source *self_component_class) {
  return bt_self_component_class_source_as_component_class_source(
      self_component_class);
}
const bt_component_class_filter *
bt_self_component_class_filter_as_component_class_filter_inline(
    bt_self_component_class_filter *self_component_class) {
  return bt_self_component_class_filter_as_component_class_filter(
      self_component_class);
}
const bt_component_class_sink *
bt_self_component_class_sink_as_component_class_sink_inline(
    bt_self_component_class_sink *self_component_class) {
  return bt_self_component_class_sink_as_component_class_sink(
      self_component_class);
}
bt_self_component_class *
bt_self_component_class_source_as_self_component_class_inline(
    bt_self_component_class_source *self_component_class) {
  return bt_self_component_class_source_as_self_component_class(
      self_component_class);
}
bt_self_component_class *
bt_self_component_class_filter_as_self_component_class_inline(
    bt_self_component_class_filter *self_component_class) {
  return bt_self_component_class_filter_as_self_component_class(
      self_component_class);
}
bt_self_component_class *
bt_self_component_class_sink_as_self_component_class_inline(
    bt_self_component_class_sink *self_component_class) {
  return bt_self_component_class_sink_as_self_component_class(
      self_component_class);
}
const bt_query_executor *
bt_private_query_executor_as_query_executor_const_inline(
    bt_private_query_executor *query_executor) {
  return bt_private_query_executor_as_query_executor_const(query_executor);
}

bt_bool
bt_component_class_is_source_inline(const bt_component_class *component_class) {
  return bt_component_class_is_source(component_class);
}

bt_bool
bt_component_class_is_filter_inline(const bt_component_class *component_class) {
  return bt_component_class_is_filter(component_class);
}

bt_bool
bt_component_class_is_sink_inline(const bt_component_class *component_class) {
  return bt_component_class_is_sink(component_class);
}

const bt_component_class *
bt_component_class_source_as_component_class_const_inline(
    const bt_component_class_source *component_class) {
  return bt_component_class_source_as_component_class_const(component_class);
}

const bt_component_class *
bt_component_class_filter_as_component_class_const_inline(
    const bt_component_class_filter *component_class) {
  return bt_component_class_filter_as_component_class_const(component_class);
}

const bt_component_class *
bt_component_class_sink_as_component_class_const_inline(
    const bt_component_class_sink *component_class) {
  return bt_component_class_sink_as_component_class_const(component_class);
}

bt_bool bt_port_is_input_inline(const bt_port *port) {
  return bt_port_is_input(port);
}

bt_bool bt_port_is_output_inline(const bt_port *port) {
  return bt_port_is_output(port);
}

const bt_port *bt_port_input_as_port_const_inline(const bt_port_input *port) {
  return bt_port_input_as_port_const(port);
}

const bt_port *bt_port_output_as_port_const_inline(const bt_port_output *port) {
  return bt_port_output_as_port_const(port);
}

bt_bool bt_component_is_source_inline(const bt_component *component) {
  return bt_component_is_source(component);
}

bt_bool bt_component_is_filter_inline(const bt_component *component) {
  return bt_component_is_filter(component);
}

bt_bool bt_component_is_sink_inline(const bt_component *component) {
  return bt_component_is_sink(component);
}

const bt_component *bt_component_source_as_component_const_inline(
    const bt_component_source *component) {
  return bt_component_source_as_component_const(component);
}

const bt_component *bt_component_filter_as_component_const_inline(
    const bt_component_filter *component) {
  return bt_component_filter_as_component_const(component);
}

const bt_component *bt_component_sink_as_component_const_inline(
    const bt_component_sink *component) {
  return bt_component_sink_as_component_const(component);
}