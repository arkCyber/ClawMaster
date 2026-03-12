Name:           clawmaster
Version:        0.1.0
Release:        1%{?dist}
Summary:        Personal AI gateway inspired by OpenClaw
License:        MIT
URL:            https://www.clawmaster.org/

%description
ClawMaster is a personal AI gateway inspired by OpenClaw. One binary, multiple LLM providers.

%install
mkdir -p %{buildroot}%{_bindir}
install -m 755 %{_sourcedir}/clawmaster %{buildroot}%{_bindir}/clawmaster

%files
%{_bindir}/clawmaster
