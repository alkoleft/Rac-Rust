from dataclasses import dataclass
from typing import Any, Dict, List, Optional


@dataclass(frozen=True, order=True)
class Version:
    major: int
    minor: int

    def __str__(self) -> str:
        return f"{self.major}.{self.minor}"


@dataclass(frozen=True)
class VersionRange:
    start: Version
    end: Optional[Version] = None

    def contains(self, version: Version) -> bool:
        if version < self.start:
            return False
        if self.end is not None and version >= self.end:
            return False
        return True

    def __str__(self) -> str:
        if self.end is None:
            return str(self.start)
        return f"{self.start}-{self.end}"


@dataclass
class FieldSpec:
    name: str
    type_name: str
    item: Optional[str] = None
    length: Optional[int] = None
    len_source: Optional[str] = None
    skip: bool = False
    computed: Optional[str] = None
    source: Optional[str] = None
    rust_type: Optional[str] = None
    literal: Optional[List[int]] = None
    version: VersionRange = None  # type: ignore[assignment]
    optional: bool = False


@dataclass
class RecordSpec:
    name: str
    derives: List[str]
    fields: List[FieldSpec]


@dataclass
class RequestSpec:
    name: str
    derives: List[str]
    fields: List[FieldSpec]


@dataclass
class RpcTestSpec:
    name: str
    hex_path: str
    args: Dict[str, Any]
    protocol: str


@dataclass
class RpcSpec:
    name: str
    request: Optional[str]
    response: Optional[str]
    request_inline: Optional[RequestSpec]
    method_req: int
    method_resp: Optional[int]
    requires_cluster_context: bool
    requires_infobase_context: bool
    tests: List[RpcTestSpec]
    version: VersionRange


@dataclass
class ResponseAssertSpec:
    field: str
    value: Any
    index: Optional[int] = None


@dataclass
class ResponseTestSpec:
    name: str
    hex_path: str
    expect_len: Optional[int]
    asserts: List[ResponseAssertSpec]
    tail_len: Optional[int] = None


@dataclass
class ResponseBodySpec:
    type_name: str
    item: Optional[str]
    tail_len_param: Optional[str] = None
    field_name: Optional[str] = None
    make_struct: Optional[bool] = None


@dataclass
class ResponseSpec:
    name: str
    body: ResponseBodySpec
    tests: List[ResponseTestSpec]
