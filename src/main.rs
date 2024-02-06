use std::{
	collections::BTreeMap,
	env::{args_os, current_dir, current_exe, vars_os},
	ffi::OsString,
};

use crossterm::style::Stylize;
use sysinfo::{get_current_pid, Pid, System};

fn main() {
	let exe = current_exe();
	let dir = current_dir();
	let args: Vec<_> = args_os().collect();
	let vars: BTreeMap<_, _> = vars_os().collect();
	let system = System::new_all();
	let pid = get_current_pid();
	eprintln!(
		"{} {exe:?} [pid={pid:?}] {} {dir:?}\n",
		"Executed as".bold(),
		"in".bold(),
		exe = current_exe(),
		dir = current_dir(),
		pid = pid.map(Pid::as_u32),
	);
	eprintln!("{} {args:#?}\n", "Arguments:".bold());
	eprintln!("{} {vars:#?}\n", "Environment:".bold());
	if let Ok(pid) = pid {
		let proc = system
			.process(pid)
			.expect("the current process should exist");
		#[cfg(debug_assertions)]
		{
			assert_eq!(proc.pid(), pid);
			assert_eq!(
				proc.environ()
					.iter()
					.map(|env| env.split_once('=').expect("env should have a = somewhere"))
					.map(|(k, v)| (k.into(), v.into()))
					.collect::<BTreeMap<_, _>>(),
				vars
			);
			dbg!(proc.environ());
			let env = proc
				.environ()
				.iter()
				.map(|env| env.split_once('=').expect("env should have a = somewhere"))
				.map(|(k, v)| (k.into(), v.into()))
				.collect::<BTreeMap<_, _>>();
			eprintln!(
				"Only `vars`: {:#?}",
				vars.iter()
					.filter(|(k, _)| !env.contains_key(*k))
					.collect::<Vec<_>>()
			);
			eprintln!(
				"Only `env`: {:#?}",
				env.iter()
					.filter(|(k, _)| !vars.contains_key(*k))
					.collect::<Vec<_>>()
			);
			eprintln!(
				"Both: {:#?}",
				vars.iter()
					.filter_map(|(k, v)| env.get(k).zip(Some(v)))
					.filter(|(v, e)| v != e)
					.collect::<Vec<_>>()
			);
			assert_eq!(env, vars);
			assert_eq!(
				proc.cmd().iter().map(OsString::from).collect::<Vec<_>>(),
				args
			);
			dbg!(proc);
			assert_eq!(proc.exe(), exe.ok().as_deref());
			assert_eq!(proc.cwd(), dir.ok().as_deref());
		}
		eprint!("{} ", "Parent:".bold());
		if let Some(ppid) = proc.parent() {
			let parent = system
				.process(ppid)
				.expect("the parent process should exist when ppid exists");
			eprintln!(
				"{name:?} ({exe:?}) [pid={ppid}] in {pwd:?}",
				name = parent.name(),
				exe = parent.exe(),
				pwd = parent.cwd(),
			);
			eprintln!(
				"{} {args:#?}",
				"Parent arguments:".bold(),
				args = parent.cmd()
			);
			eprintln!("{}", "Siblings:".bold());
			for (pid, v) in system
				.processes()
				.iter()
				.filter(|(k, v)| **k != pid && v.parent() == Some(ppid))
			{
				eprintln!(
					"    {name:?} ({exe:?}) [pid={pid}] in {cwd:?}",
					name = v.name(),
					exe = v.exe(),
					cwd = v.cwd()
				);
			}
		} else {
			eprintln!("{:?}", None::<Pid>);
		};
	}
}
