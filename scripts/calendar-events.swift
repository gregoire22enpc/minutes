#!/usr/bin/env swift
// Queries macOS Calendar via EventKit and prints upcoming events as JSON lines.
// Usage: swift calendar-events.swift [lookahead_minutes]
// Output: one JSON object per line: {"title":"...","start":"...","minutes_until":N}

import EventKit
import Foundation

let lookaheadMinutes = Int(CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : "240") ?? 240
let store = EKEventStore()
let semaphore = DispatchSemaphore(value: 0)

store.requestFullAccessToEvents { granted, error in
    defer { semaphore.signal() }
    guard granted else {
        // If full access fails, try read-only (older macOS)
        return
    }

    let now = Date()
    guard let end = Calendar.current.date(byAdding: .minute, value: lookaheadMinutes, to: now) else { return }
    let predicate = store.predicateForEvents(withStart: now, end: end, calendars: nil)
    let events = store.events(matching: predicate)
        .filter { !$0.isAllDay }
        .sorted { $0.startDate < $1.startDate }

    for event in events {
        let mins = Int(event.startDate.timeIntervalSince(now) / 60)
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd HH:mm"
        let startStr = formatter.string(from: event.startDate)
        let title = (event.title ?? "Untitled")
            .replacingOccurrences(of: "\\", with: "\\\\")
            .replacingOccurrences(of: "\"", with: "\\\"")
            .replacingOccurrences(of: "\n", with: " ")
        print("{\"title\":\"\(title)\",\"start\":\"\(startStr)\",\"minutes_until\":\(mins)}")
    }
}

semaphore.wait()
