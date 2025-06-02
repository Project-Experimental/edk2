import os
from SCons.Script import Alias, DefaultEnvironment, Command, Default

ARCH = 'X64'
TOOLCHAIN = 'GCC5'
BUILD_TYPE = 'DEBUG'
PACKAGE = 'OvmfPkg/OvmfPkgX64.dsc'

EDK2_ROOT = Dir('.').abspath

env = DefaultEnvironment(ENV = os.environ)

base_tools = Command(
    'BaseToolsBuilt.log',
    [],
    [
        "make -C %s/BaseTools" % EDK2_ROOT
    ]
)

def build_edk2(target, source, env):
    cmd = [
        "source %s/edksetup.sh" % EDK2_ROOT,
        "build -a %s -t %s -b %s" % (ARCH, TOOLCHAIN, BUILD_TYPE)
    ]
    return env.Execute(" && ".join(cmd))

cmd = [
    ". %s/edksetup.sh" % EDK2_ROOT,
    "build -p %s -a %s -t %s -b %s" % (PACKAGE, ARCH, TOOLCHAIN, BUILD_TYPE)
]
# build_command

build_edk = Command(
    'BaseToolsBuilt.log',
    [],
    [
        " && ".join(cmd)
    ]  
)

Alias('basetools', base_tools)

Default(build_edk)
