/* Automatically generated by sc-gen 0.1.0 */

// pub const _LLSEEK: usize = __NR__llseek;
pub const _NEWSELECT: usize = 5022;
pub const _SYSCTL: usize = 5152;
pub const ACCEPT: usize = 5042;
pub const ACCEPT4: usize = 5293;
pub const ACCESS: usize = 5020;
pub const ACCT: usize = 5158;
pub const ADD_KEY: usize = 5239;
pub const ADJTIMEX: usize = 5154;
pub const ALARM: usize = 5037;
// pub const ARM_FADVISE64_64: usize = __NR_arm_fadvise64_64;
// pub const ARM_SYNC_FILE_RANGE: usize = __NR_arm_sync_file_range;
// pub const BDFLUSH: usize = __NR_bdflush;
pub const BIND: usize = 5048;
pub const BRK: usize = 5012;
pub const CAPGET: usize = 5123;
pub const CAPSET: usize = 5124;
pub const CHDIR: usize = 5078;
pub const CHMOD: usize = 5088;
pub const CHOWN: usize = 5090;
// pub const CHOWN32: usize = __NR_chown32;
pub const CHROOT: usize = 5156;
pub const CLOCK_ADJTIME: usize = 5300;
pub const CLOCK_GETRES: usize = 5223;
pub const CLOCK_GETTIME: usize = 5222;
pub const CLOCK_NANOSLEEP: usize = 5224;
pub const CLOCK_SETTIME: usize = 5221;
pub const CLONE: usize = 5055;
pub const CLOSE: usize = 5003;
pub const CONNECT: usize = 5041;
pub const CREAT: usize = 5083;
pub const DELETE_MODULE: usize = 5169;
pub const DUP: usize = 5031;
pub const DUP2: usize = 5032;
pub const DUP3: usize = 5286;
pub const EPOLL_CREATE: usize = 5207;
pub const EPOLL_CREATE1: usize = 5285;
pub const EPOLL_CTL: usize = 5208;
pub const EPOLL_PWAIT: usize = 5272;
pub const EPOLL_WAIT: usize = 5209;
pub const EVENTFD: usize = 5278;
pub const EVENTFD2: usize = 5284;
pub const EXECVE: usize = 5057;
pub const EXIT: usize = 5058;
pub const EXIT_GROUP: usize = 5205;
pub const FACCESSAT: usize = 5259;
pub const FALLOCATE: usize = 5279;
pub const FANOTIFY_INIT: usize = 5295;
pub const FANOTIFY_MARK: usize = 5296;
pub const FCHDIR: usize = 5079;
pub const FCHMOD: usize = 5089;
pub const FCHMODAT: usize = 5258;
pub const FCHOWN: usize = 5091;
// pub const FCHOWN32: usize = __NR_fchown32;
pub const FCHOWNAT: usize = 5250;
pub const FCNTL: usize = 5070;
// pub const FCNTL64: usize = __NR_fcntl64;
pub const FDATASYNC: usize = 5073;
pub const FGETXATTR: usize = 5185;
pub const FINIT_MODULE: usize = 5307;
pub const FLISTXATTR: usize = 5188;
pub const FLOCK: usize = 5071;
pub const FORK: usize = 5056;
pub const FREMOVEXATTR: usize = 5191;
pub const FSETXATTR: usize = 5182;
pub const FSTAT: usize = 5005;
// pub const FSTAT64: usize = __NR_fstat64;
// pub const FSTATAT64: usize = __NR_fstatat64;
pub const FSTATFS: usize = 5135;
// pub const FSTATFS64: usize = __NR_fstatfs64;
pub const FSYNC: usize = 5072;
pub const FTRUNCATE: usize = 5075;
// pub const FTRUNCATE64: usize = __NR_ftruncate64;
pub const FUTEX: usize = 5194;
pub const FUTIMESAT: usize = 5251;
pub const GET_MEMPOLICY: usize = 5228;
pub const GET_ROBUST_LIST: usize = 5269;
pub const GETCPU: usize = 5271;
pub const GETCWD: usize = 5077;
pub const GETDENTS: usize = 5076;
pub const GETDENTS64: usize = 5308;
pub const GETEGID: usize = 5106;
// pub const GETEGID32: usize = __NR_getegid32;
pub const GETEUID: usize = 5105;
// pub const GETEUID32: usize = __NR_geteuid32;
pub const GETGID: usize = 5102;
// pub const GETGID32: usize = __NR_getgid32;
pub const GETGROUPS: usize = 5113;
// pub const GETGROUPS32: usize = __NR_getgroups32;
pub const GETITIMER: usize = 5035;
pub const GETPEERNAME: usize = 5051;
pub const GETPGID: usize = 5119;
pub const GETPGRP: usize = 5109;
pub const GETPID: usize = 5038;
pub const GETPPID: usize = 5108;
pub const GETPRIORITY: usize = 5137;
pub const GETRESGID: usize = 5118;
// pub const GETRESGID32: usize = __NR_getresgid32;
pub const GETRESUID: usize = 5116;
// pub const GETRESUID32: usize = __NR_getresuid32;
pub const GETRLIMIT: usize = 5095;
pub const GETRUSAGE: usize = 5096;
pub const GETSID: usize = 5122;
pub const GETSOCKNAME: usize = 5050;
pub const GETSOCKOPT: usize = 5054;
pub const GETTID: usize = 5178;
pub const GETTIMEOFDAY: usize = 5094;
pub const GETUID: usize = 5100;
// pub const GETUID32: usize = __NR_getuid32;
pub const GETXATTR: usize = 5183;
pub const INIT_MODULE: usize = 5168;
pub const INOTIFY_ADD_WATCH: usize = 5244;
pub const INOTIFY_INIT: usize = 5243;
pub const INOTIFY_INIT1: usize = 5288;
pub const INOTIFY_RM_WATCH: usize = 5245;
pub const IO_CANCEL: usize = 5204;
pub const IO_DESTROY: usize = 5201;
pub const IO_GETEVENTS: usize = 5202;
pub const IO_SETUP: usize = 5200;
pub const IO_SUBMIT: usize = 5203;
pub const IOCTL: usize = 5015;
pub const IOPRIO_GET: usize = 5274;
pub const IOPRIO_SET: usize = 5273;
// pub const IPC: usize = __NR_ipc;
pub const KCMP: usize = 5306;
pub const KEXEC_LOAD: usize = 5270;
pub const KEYCTL: usize = 5241;
pub const KILL: usize = 5060;
pub const LCHOWN: usize = 5092;
// pub const LCHOWN32: usize = __NR_lchown32;
pub const LGETXATTR: usize = 5184;
pub const LINK: usize = 5084;
pub const LINKAT: usize = 5255;
pub const LISTEN: usize = 5049;
pub const LISTXATTR: usize = 5186;
pub const LLISTXATTR: usize = 5187;
pub const LOOKUP_DCOOKIE: usize = 5206;
pub const LREMOVEXATTR: usize = 5190;
pub const LSEEK: usize = 5008;
pub const LSETXATTR: usize = 5181;
pub const LSTAT: usize = 5006;
// pub const LSTAT64: usize = __NR_lstat64;
pub const MADVISE: usize = 5027;
pub const MBIND: usize = 5227;
pub const MINCORE: usize = 5026;
pub const MKDIR: usize = 5081;
pub const MKDIRAT: usize = 5248;
pub const MKNOD: usize = 5131;
pub const MKNODAT: usize = 5249;
pub const MLOCK: usize = 5146;
pub const MLOCKALL: usize = 5148;
pub const MMAP: usize = 5009;
// pub const MMAP2: usize = __NR_mmap2;
pub const MOUNT: usize = 5160;
pub const MOVE_PAGES: usize = 5267;
pub const MPROTECT: usize = 5010;
pub const MQ_GETSETATTR: usize = 5235;
pub const MQ_NOTIFY: usize = 5234;
pub const MQ_OPEN: usize = 5230;
pub const MQ_TIMEDRECEIVE: usize = 5233;
pub const MQ_TIMEDSEND: usize = 5232;
pub const MQ_UNLINK: usize = 5231;
pub const MREMAP: usize = 5024;
pub const MSGCTL: usize = 5069;
pub const MSGGET: usize = 5066;
pub const MSGRCV: usize = 5068;
pub const MSGSND: usize = 5067;
pub const MSYNC: usize = 5025;
pub const MUNLOCK: usize = 5147;
pub const MUNLOCKALL: usize = 5149;
pub const MUNMAP: usize = 5011;
pub const NAME_TO_HANDLE_AT: usize = 5298;
pub const NANOSLEEP: usize = 5034;
pub const NFSSERVCTL: usize = 5173;
// pub const NICE: usize = __NR_nice;
pub const OPEN: usize = 5002;
pub const OPEN_BY_HANDLE_AT: usize = 5299;
pub const OPENAT: usize = 5247;
pub const PAUSE: usize = 5033;
// pub const PCICONFIG_IOBASE: usize = __NR_pciconfig_iobase;
// pub const PCICONFIG_READ: usize = __NR_pciconfig_read;
// pub const PCICONFIG_WRITE: usize = __NR_pciconfig_write;
pub const PERF_EVENT_OPEN: usize = 5292;
pub const PERSONALITY: usize = 5132;
pub const PIPE: usize = 5021;
pub const PIPE2: usize = 5287;
pub const PIVOT_ROOT: usize = 5151;
pub const POLL: usize = 5007;
pub const PPOLL: usize = 5261;
pub const PRCTL: usize = 5153;
pub const PREAD64: usize = 5016;
pub const PREADV: usize = 5289;
pub const PRLIMIT64: usize = 5297;
pub const PROCESS_VM_READV: usize = 5304;
pub const PROCESS_VM_WRITEV: usize = 5305;
pub const PSELECT6: usize = 5260;
pub const PTRACE: usize = 5099;
pub const PWRITE64: usize = 5017;
pub const PWRITEV: usize = 5290;
pub const QUOTACTL: usize = 5172;
pub const READ: usize = 5000;
pub const READAHEAD: usize = 5179;
// pub const READDIR: usize = __NR_readdir;
pub const READLINK: usize = 5087;
pub const READLINKAT: usize = 5257;
pub const READV: usize = 5018;
pub const REBOOT: usize = 5164;
// pub const RECV: usize = __NR_recv;
pub const RECVFROM: usize = 5044;
pub const RECVMMSG: usize = 5294;
pub const RECVMSG: usize = 5046;
pub const REMAP_FILE_PAGES: usize = 5210;
pub const REMOVEXATTR: usize = 5189;
pub const RENAME: usize = 5080;
pub const RENAMEAT: usize = 5254;
pub const REQUEST_KEY: usize = 5240;
pub const RESTART_SYSCALL: usize = 5213;
pub const RMDIR: usize = 5082;
pub const RT_SIGACTION: usize = 5013;
pub const RT_SIGPENDING: usize = 5125;
pub const RT_SIGPROCMASK: usize = 5014;
pub const RT_SIGQUEUEINFO: usize = 5127;
pub const RT_SIGRETURN: usize = 5211;
pub const RT_SIGSUSPEND: usize = 5128;
pub const RT_SIGTIMEDWAIT: usize = 5126;
pub const RT_TGSIGQUEUEINFO: usize = 5291;
pub const SCHED_GET_PRIORITY_MAX: usize = 5143;
pub const SCHED_GET_PRIORITY_MIN: usize = 5144;
pub const SCHED_GETAFFINITY: usize = 5196;
pub const SCHED_GETPARAM: usize = 5140;
pub const SCHED_GETSCHEDULER: usize = 5142;
pub const SCHED_RR_GET_INTERVAL: usize = 5145;
pub const SCHED_SETAFFINITY: usize = 5195;
pub const SCHED_SETPARAM: usize = 5139;
pub const SCHED_SETSCHEDULER: usize = 5141;
pub const SCHED_YIELD: usize = 5023;
// pub const SELECT: usize = __NR_select;
pub const SEMCTL: usize = 5064;
pub const SEMGET: usize = 5062;
pub const SEMOP: usize = 5063;
pub const SEMTIMEDOP: usize = 5214;
// pub const SEND: usize = __NR_send;
pub const SENDFILE: usize = 5039;
// pub const SENDFILE64: usize = __NR_sendfile64;
pub const SENDMMSG: usize = 5302;
pub const SENDMSG: usize = 5045;
pub const SENDTO: usize = 5043;
pub const SET_MEMPOLICY: usize = 5229;
pub const SET_ROBUST_LIST: usize = 5268;
pub const SET_TID_ADDRESS: usize = 5212;
pub const SETDOMAINNAME: usize = 5166;
pub const SETFSGID: usize = 5121;
// pub const SETFSGID32: usize = __NR_setfsgid32;
pub const SETFSUID: usize = 5120;
// pub const SETFSUID32: usize = __NR_setfsuid32;
pub const SETGID: usize = 5104;
// pub const SETGID32: usize = __NR_setgid32;
pub const SETGROUPS: usize = 5114;
// pub const SETGROUPS32: usize = __NR_setgroups32;
pub const SETHOSTNAME: usize = 5165;
pub const SETITIMER: usize = 5036;
pub const SETNS: usize = 5303;
pub const SETPGID: usize = 5107;
pub const SETPRIORITY: usize = 5138;
pub const SETREGID: usize = 5112;
// pub const SETREGID32: usize = __NR_setregid32;
pub const SETRESGID: usize = 5117;
// pub const SETRESGID32: usize = __NR_setresgid32;
pub const SETRESUID: usize = 5115;
// pub const SETRESUID32: usize = __NR_setresuid32;
pub const SETREUID: usize = 5111;
// pub const SETREUID32: usize = __NR_setreuid32;
pub const SETRLIMIT: usize = 5155;
pub const SETSID: usize = 5110;
pub const SETSOCKOPT: usize = 5053;
pub const SETTIMEOFDAY: usize = 5159;
pub const SETUID: usize = 5103;
// pub const SETUID32: usize = __NR_setuid32;
pub const SETXATTR: usize = 5180;
pub const SHMAT: usize = 5029;
pub const SHMCTL: usize = 5030;
pub const SHMDT: usize = 5065;
pub const SHMGET: usize = 5028;
pub const SHUTDOWN: usize = 5047;
// pub const SIGACTION: usize = __NR_sigaction;
pub const SIGALTSTACK: usize = 5129;
pub const SIGNALFD: usize = 5276;
pub const SIGNALFD4: usize = 5283;
// pub const SIGPENDING: usize = __NR_sigpending;
// pub const SIGPROCMASK: usize = __NR_sigprocmask;
// pub const SIGRETURN: usize = __NR_sigreturn;
// pub const SIGSUSPEND: usize = __NR_sigsuspend;
pub const SOCKET: usize = 5040;
// pub const SOCKETCALL: usize = __NR_socketcall;
pub const SOCKETPAIR: usize = 5052;
pub const SPLICE: usize = 5263;
pub const STAT: usize = 5004;
// pub const STAT64: usize = __NR_stat64;
pub const STATFS: usize = 5134;
// pub const STATFS64: usize = __NR_statfs64;
// pub const STIME: usize = __NR_stime;
pub const SWAPOFF: usize = 5163;
pub const SWAPON: usize = 5162;
pub const SYMLINK: usize = 5086;
pub const SYMLINKAT: usize = 5256;
pub const SYNC: usize = 5157;
// pub const SYNC_FILE_RANGE2: usize = __NR_sync_file_range2;
pub const SYNCFS: usize = 5301;
// pub const SYSCALL: usize = __NR_syscall;
pub const SYSFS: usize = 5136;
pub const SYSINFO: usize = 5097;
pub const SYSLOG: usize = 5101;
pub const TEE: usize = 5265;
pub const TGKILL: usize = 5225;
// pub const TIME: usize = __NR_time;
pub const TIMER_CREATE: usize = 5216;
pub const TIMER_DELETE: usize = 5220;
pub const TIMER_GETOVERRUN: usize = 5219;
pub const TIMER_GETTIME: usize = 5218;
pub const TIMER_SETTIME: usize = 5217;
pub const TIMERFD_CREATE: usize = 5280;
pub const TIMERFD_GETTIME: usize = 5281;
pub const TIMERFD_SETTIME: usize = 5282;
pub const TIMES: usize = 5098;
pub const TKILL: usize = 5192;
pub const TRUNCATE: usize = 5074;
// pub const TRUNCATE64: usize = __NR_truncate64;
// pub const UGETRLIMIT: usize = __NR_ugetrlimit;
pub const UMASK: usize = 5093;
// pub const UMOUNT: usize = __NR_umount;
pub const UMOUNT2: usize = 5161;
pub const UNAME: usize = 5061;
pub const UNLINK: usize = 5085;
pub const UNLINKAT: usize = 5253;
pub const UNSHARE: usize = 5262;
// pub const USELIB: usize = __NR_uselib;
pub const USTAT: usize = 5133;
pub const UTIME: usize = 5130;
pub const UTIMENSAT: usize = 5275;
pub const UTIMES: usize = 5226;
// pub const VFORK: usize = __NR_vfork;
pub const VHANGUP: usize = 5150;
pub const VMSPLICE: usize = 5266;
pub const VSERVER: usize = 5236;
pub const WAIT4: usize = 5059;
pub const WAITID: usize = 5237;
pub const WRITE: usize = 5001;
pub const WRITEV: usize = 5019;
