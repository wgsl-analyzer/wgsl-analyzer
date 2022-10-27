// logical
all(vecN<bool>) -> bool 
all(bool) -> bool 
any(vecN<bool>) -> bool 
any(bool) -> bool 
select(T, T, bool) -> T
select(vecN<T>, vecN<T>, vecN<bool>) -> vecN<T>

// value-testing
isNan(f32) -> bool
isNan(vecN<f32>) -> vecN<bool>
isInf(f32) -> bool
isInf(vecN<f32>) -> vecN<bool>
isFinite(f32) -> bool
isFinite(vecN<f32>) -> vecN<bool>
isNormal(f32) -> bool
isNormal(vecN<f32>) -> vecN<bool>
arrayLength(ptr<array<T>>) -> u32

// float
abs(f32) -> f32
abs(vecN<f32>) -> vecN<f32>
acos(f32) -> f32
acos(vecN<f32>) -> vecN<f32>
asin(f32) -> f32
asin(vecN<f32>) -> vecN<f32>
atan(f32) -> f32
atan(vecN<f32>) -> vecN<f32>
atan2(f32, f32) -> f32
atan2(vecN<f32>, vecN<f32>) -> vecN<f32>
ceil(f32) -> f32
ceil(vecN<f32>) -> vecN<f32>
clamp(f32, f32, f32) -> f32
clamp(vecN<f32>, vecN<f32>, vecN<f32>) -> vecN<f32>
cos(f32) -> f32
cos(vecN<f32>) -> vecN<f32>
cosh(f32) -> f32
cosh(vecN<f32>) -> vecN<f32>
cross(vec3<f32>, vec3<f32>) -> vec3<f32>
distance(f32, f32) -> f32
distance(vecN<f32>, vecN<f32>) -> f32
exp(f32) -> f32
exp(vecN<f32>) -> vecN<f32>
exp2(f32) -> f32
exp2(vecN<f32>) -> vecN<f32>
faceForward(vecN<f32>, vecN<f32>, vecN<f32>) -> vecN<f32>
floor(f32) -> f32
floor(vecN<f32>) -> vecN<f32>
fma(f32, f32) -> f32
fma(vecN<f32>, vecN<f32>) -> f32
fract(f32) -> f32
fract(vecN<f32>) -> vecN<f32>
// TODO frexp
inverseSqrt(f32) -> f32
inverseSqrt(vecN<f32>) -> vecN<f32>
// TODO ldexp
length(f32) -> f32
length(vecN<f32>) -> f32
log(f32) -> f32
log(vecN<f32>) -> vecN<f32>
log2(f32) -> f32
log2(vecN<f32>) -> vecN<f32>
min(f32, f32) -> f32
min(vecN<f32>, vecN<f32>) -> vecN<f32>
max(f32, f32) -> f32
max(vecN<f32>, vecN<f32>) -> vecN<f32>
mix(f32, f32, f32) -> f32
mix(vecN<f32>, vecN<f32>, vecN<f32>) -> vecN<f32>
mix(vecN<f32>, vecN<f32>, f32) -> vecN<f32>
// TODO modf
normalize(vecN<f32>) -> vecN<f32>
pow(f32, f32) -> f32
pow(vecN<f32>, vecN<f32>) -> vecN<f32>
quantizeToF16(f32) -> f32
quantizeToF16(vecN<f32>) -> vecN<f32>
reflect(vecN<f32>, vecN<f32>) -> vecN<f32>
refract(vecN<f32>, vecN<f32>, f32) -> f32
round(f32) -> f32
round(vecN<f32>) -> vecN<f32>
sign(f32) -> f32
sign(vecN<f32>) -> vecN<f32>
sin(f32) -> f32
sin(vecN<f32>) -> vecN<f32>
sinh(f32) -> f32
sinh(vecN<f32>) -> vecN<f32>
smoothstep(f32, f32, f32) -> f32
smoothstep(vecN<f32>, vecN<f32>, vecN<f32>) -> vecN<f32>
sqrt(f32) -> f32
sqrt(vecN<f32>) -> vecN<f32>
step(f32, f32) -> f32
step(vecN<f32>, vecN<f32>) -> vecN<f32>
tan(f32) -> f32
tan(vecN<f32>) -> vecN<f32>
tanh(f32) -> f32
tanh(vecN<f32>) -> vecN<f32>
trunc(f32) -> f32
trunc(vecN<f32>) -> vecN<f32>

// integer
abs(i32) -> i32
abs(vecN<i32>) -> vecN<i32>
abs(u32) -> u32
abs(vecN<u32>) -> vecN<u32>
clamp(i32, i32, i32) -> i32
clamp(vecN<i32>, vecN<i32>, vecN<i32>) -> vecN<i32>
clamp(u32, u32, u32) -> u32
clamp(vecN<u32>, vecN<u32>, vecN<u32>) -> vecN<u32>
countLeadingZeros(i32) -> i32
countLeadingZeros(vecN<i32>) -> vecN<i32>
countLeadingZeros(u32) -> u32
countLeadingZeros(vecN<u32>) -> vecN<u32>
countOneBits(i32) -> i32
countOneBits(vecN<i32>) -> vecN<i32>
countOneBits(u32) -> u32
countOneBits(vecN<u32>) -> vecN<u32>
countTrailingZeros(i32) -> i32
countTrailingZeros(vecN<i32>) -> vecN<i32>
countTrailingZeros(u32) -> u32
countTrailingZeros(vecN<u32>) -> vecN<u32>
firstLeadingBit(i32) -> i32
firstLeadingBit(vecN<i32>) -> vecN<i32>
firstLeadingBit(u32) -> u32
firstLeadingBit(vecN<u32>) -> vecN<u32>
firstTrailingBit(i32) -> i32
firstTrailingBit(vecN<i32>) -> vecN<i32>
firstTrailingBit(u32) -> u32
firstTrailingBit(vecN<u32>) -> vecN<u32>
extractBits(i32, u32, u32) -> i32
extractBits(vecN<i32>, u32, u32) -> vecN<i32>
extractBits(u32, u32, u32) -> u32
extractBits(vecN<u32>, u32, u32) -> vecN<i32>
insertBits(i32, i32, u32, u32) -> i32
insertBits(vecN<i32>, vecN<i32>, u32, u32) -> vecN<i32>
insertBits(u32, u32, u32, u32) -> u32
insertBits(vecN<u32>, vecN<u32>, u32, u32) -> vecN<u32>
max(i32, i32) -> i32
max(vecN<i32>, vecN<i32>) -> vecN<i32>
max(u32, u32) -> u32
max(vecN<u32>, vecN<u32>) -> vecN<u32>
min(i32, i32) -> i32
min(vecN<i32>, vecN<i32>) -> vecN<i32>
min(u32, u32) -> u32
min(vecN<u32>, vecN<u32>) -> vecN<u32>
reverseBits(i32) -> i32
reverseBits(vecN<i32>) -> vecN<i32>
reverseBits(u32) -> u32
reverseBits(vecN<u32>) -> vecN<u32>

// matrix
determinant(matNxN<T>) -> T
transpose(matMxN<T>) -> matNxM<T>

// vector
dot(vecN<f32>, vecN<f32>) -> f32

// derivative
dpdx(f32) -> f32
dpdx(vecN<f32>) -> vecN<f32>
dpdxCoarse(f32) -> f32
dpdxCoarse(vecN<f32>) -> vecN<f32>
dpdxFine(f32) -> f32
dpdxFine(vecN<f32>) -> vecN<f32>
dpdy(f32) -> f32
dpdy(vecN<f32>) -> vecN<f32>
dpdyCoarse(f32) -> f32
dpdyCoarse(vecN<f32>) -> vecN<f32>
dpdyFine(f32) -> f32
dpdyFine(vecN<f32>) -> vecN<f32>
fwidth(f32) -> f32
fwidth(vecN<f32>) -> vecN<f32>
fwidthCoarse(f32) -> f32
fwidthCoarse(vecN<f32>) -> vecN<f32>
fwidthFine(f32) -> f32
fwidthFine(vecN<f32>) -> vecN<f32>

// texture
textureDimensions(texture_1d<T>) -> i32
textureDimensions(texture_1d<T>, i32) -> i32
textureDimensions(texture_2d<T>) -> vec2<i32>
textureDimensions(texture_2d<T>, i32) -> vec2<i32>
textureDimensions(texture_2d_array<T>) -> vec2<i32>
textureDimensions(texture_2d_array<T>, i32) -> vec2<i32>
textureDimensions(texture_3d<T>) -> vec3<i32>
textureDimensions(texture_3d<T>, i32) -> vec3<i32>
textureDimensions(texture_cube<T>) -> vec2<i32>
textureDimensions(texture_cube<T>, i32) -> vec2<i32>
textureDimensions(texture_cube_array<T>) -> vec2<i32>
textureDimensions(texture_cube_array<T>, i32) -> vec2<i32>
textureDimensions(texture_multisampled_2d<T>) -> vec2<i32>
textureDimensions(texture_depth_2d) -> vec2<i32>
textureDimensions(texture_depth_2d, i32) -> vec2<i32>
textureDimensions(texture_depth_2d_array) -> vec2<i32>
textureDimensions(texture_depth_2d_array, i32) -> vec2<i32>
textureDimensions(texture_depth_cube) -> vec2<i32>
textureDimensions(texture_depth_cube, i32) -> vec2<i32>
textureDimensions(texture_depth_cube_array) -> vec2<i32>
textureDimensions(texture_depth_cube_array, i32) -> vec2<i32>
textureDimensions(texture_depth_multisampled_2d) -> vec2<i32>
textureDimensions(texture_storage_1d<_;_>) -> i32
textureDimensions(texture_storage_2d<_;_>) -> vec2<i32>
textureDimensions(texture_storage_2d_array<_;_>) -> vec2<i32>
textureDimensions(texture_storage_3d<_;_>) -> vec3<i32>
textureDimensions(texture_external) -> vec2<i32>

textureLoad(texture_1d<T>, i32, i32) -> vec4<T>
textureLoad(texture_2d<T>, vec2<i32>, i32) -> vec4<T>
textureLoad(texture_2d_array<T>, vec2<i32>, i32, i32) -> vec4<T>
textureLoad(texture_3d<T>, vec3<i32>, i32) -> vec4<T>
textureLoad(texture_multisampled_2d<T>, vec2<i32>, i32) -> vec4<T>
textureLoad(texture_depth_2d, vec2<i32>, i32) -> f32
textureLoad(texture_depth_2d_array, vec2<i32>, i32, i32) -> f32
textureLoad(texture_depth_multisampled_2d, vec2<i32>, i32) -> f32
textureLoad(texture_external, vec2<i32>) -> vec4<f32>

textureLoad(texture_storage_1d<F;read>, i32) -> F::StorageType
textureLoad(texture_storage_2d<F;read>, vec2<i32>) -> F::StorageType
textureLoad(texture_storage_2d_array<F;read>, vec2<i32>, i32) -> F::StorageType
textureLoad(texture_storage_3d<F;read>, vec3<i32>) -> F::StorageType

textureGather(i32, texture_2d<T>, sampler, vec2<f32>) -> vec4<T>
textureGather(i32, texture_2d<T>, sampler, vec2<f32>, vec2<i32>) -> vec4<T>
textureGather(i32, texture_2d_array<T>, sampler, vec2<f32>, i32) -> vec4<T>
textureGather(i32, texture_2d_array<T>, sampler, vec2<f32>, i32, vec2<i32>) -> vec4<T>
textureGather(i32, texture_cube<T>, sampler, vec3<f32>) -> vec4<T>
textureGather(i32, texture_cube_array<T>, sampler, vec3<f32>, i32) -> vec4<T>
textureGather(texture_depth_2d, sampler, vec2<f32>) -> vec4<f32>
textureGather(texture_depth_2d, sampler, vec2<f32>, vec2<i32>) -> vec4<f32>
textureGather(texture_depth_2d_array, sampler, vec2<f32>, i32) -> vec4<f32>
textureGather(texture_depth_2d_array, sampler, vec2<f32>, i32, vec2<i32>) -> vec4<f32>
textureGather(texture_depth_cube, sampler, vec3<f32>) -> vec4<f32>
textureGather(texture_depth_cube_array, sampler, vec3<f32>, i32) -> vec4<f32>

textureGatherCompare(texture_depth_2d, sampler_comparison, vec2<f32>, f32) -> vec4<f32>
textureGatherCompare(texture_depth_2d, sampler_comparison, vec2<f32>, f32, vec2<i32>) -> vec4<f32>
textureGatherCompare(texture_depth_2d_array, sampler_comparison, vec2<f32>, i32, f32) -> vec4<f32>
textureGatherCompare(texture_depth_2d_array, sampler_comparison, vec2<f32>, i32, f32, vec2<i32>) -> vec4<f32>
textureGatherCompare(texture_depth_cube, sampler_comparison, vec3<f32>, f32) -> vec4<f32>
textureGatherCompare(texture_depth_cube_array, sampler_comparison, vec3<f32>, i32, f32) -> vec4<f32>


textureNumLayers(texture_2d_array<T>) -> i32
textureNumLayers(texture_cube_array<T>) -> i32
textureNumLayers(texture_depth_2d_array) -> i32
textureNumLayers(texture_depth_cube_array) -> i32
textureNumLayers(texture_storage_2d_array<_;_>) -> i32

textureNumLevels(texture_1d<T>) -> i32
textureNumLevels(texture_2d<T>) -> i32
textureNumLevels(texture_2d_array<T>) -> i32
textureNumLevels(texture_3d<T>) -> i32
textureNumLevels(texture_cube<T>) -> i32
textureNumLevels(texture_cube_array<T>) -> i32
textureNumLevels(texture_depth_2d) -> i32
textureNumLevels(texture_depth_2d_array) -> i32
textureNumLevels(texture_depth_cube) -> i32
textureNumLevels(texture_depth_cube_array) -> i32

textureNumSamples(texture_multisampled_2d<T>) -> i32
textureNumSamples(texture_depth_multisampled_2d) -> i32

textureSample(texture_1d<f32>, sampler, f32) -> vec4<f32>
textureSample(texture_2d<f32>, sampler, vec2<f32>) -> vec4<f32>
textureSample(texture_2d<f32>, sampler, vec2<f32>, vec2<i32>) -> vec4<f32>
textureSample(texture_2d_array<f32>, sampler, vec2<f32>, i32) -> vec4<f32>
textureSample(texture_2d_array<f32>, sampler, vec2<f32>, i32, vec2<i32>) -> vec4<f32>
textureSample(texture_3d<f32>, sampler, vec3<f32>) -> vec4<f32>
textureSample(texture_3d<f32>, sampler, vec3<f32>, vec3<i32>) -> vec4<f32>
textureSample(texture_cube<f32>, sampler, vec3<f32>) -> vec4<f32>
textureSample(texture_cube_array<f32>, sampler, vec3<f32>, i32) -> vec4<f32>
textureSample(texture_depth_2d, sampler, vec2<f32>) -> f32
textureSample(texture_depth_2d, sampler, vec2<f32>, vec2<i32>) -> f32
textureSample(texture_depth_2d_array, sampler, vec2<f32>, i32) -> f32
textureSample(texture_depth_2d_array, sampler, vec2<f32>, i32, vec2<i32>) -> f32
textureSample(texture_depth_cube, sampler, vec3<f32>) -> f32
textureSample(texture_depth_cube_array, sampler, vec3<f32>, i32) -> f32

textureSampleBias(texture_2d<f32>, sampler, vec2<f32>, f32) -> vec4<f32>
textureSampleBias(texture_2d<f32>, sampler, vec2<f32>, f32, vec2<i32>) -> vec4<f32>
textureSampleBias(texture_2d_array<f32>, sampler, vec2<f32>, i32, f32) -> vec4<f32>
textureSampleBias(texture_2d_array<f32>, sampler, vec2<f32>, i32, f32, vec2<i32>) -> vec4<f32>
textureSampleBias(texture_3d<f32>, sampler, vec3<f32>, f32) -> vec4<f32>
textureSampleBias(texture_3d<f32>, sampler, vec3<f32>, f32, vec3<i32>) -> vec4<f32>
textureSampleBias(texture_cube<f32>, sampler, vec3<f32>, f32) -> vec4<f32>
textureSampleBias(texture_cube_array<f32>, sampler, vec3<f32>, i32, f32) -> vec4<f32>

textureSampleCompare(texture_depth_2d, sampler_comparison, vec2<f32>, f32) -> f32
textureSampleCompare(texture_depth_2d, sampler_comparison, vec2<f32>, f32, vec2<i32>) -> f32
textureSampleCompare(texture_depth_2d_array, sampler_comparison, vec2<f32>, i32, f32) -> f32
textureSampleCompare(texture_depth_2d_array, sampler_comparison, vec2<f32>, i32, f32, vec2<i32>) -> f32
textureSampleCompare(texture_depth_cube, sampler_comparison, vec3<f32>, f32) -> f32
textureSampleCompare(texture_depth_cube_array, sampler_comparison, vec3<f32>, i32, f32) -> f32

textureSampleCompareLevel(texture_depth_2d, sampler_comparison, vec2<f32>, f32) -> f32
textureSampleCompareLevel(texture_depth_2d, sampler_comparison, vec2<f32>, f32, vec2<i32>) -> f32
textureSampleCompareLevel(texture_depth_2d_array, sampler_comparison, vec2<f32>, i32, f32) -> f32
textureSampleCompareLevel(texture_depth_2d_array, sampler_comparison, vec2<f32>, i32, f32, vec2<i32>) -> f32
textureSampleCompareLevel(texture_depth_cube, sampler_comparison, vec3<f32>, f32) -> f32
textureSampleCompareLevel(texture_depth_cube_array, sampler_comparison, vec3<f32>, i32, f32) -> f32

textureSampleGrad(texture_2d<f32>, sampler, vec2<f32>, vec2<f32>, vec2<f32>) -> vec4<f32>
textureSampleGrad(texture_2d<f32>, sampler, vec2<f32>, vec2<f32>, vec2<f32>, vec2<i32>) -> vec4<f32>
textureSampleGrad(texture_2d_array<f32>, sampler, vec2<f32>, i32, vec2<f32>, vec2<f32>) -> vec4<f32>
textureSampleGrad(texture_2d_array<f32>, sampler, vec2<f32>, i32, vec2<f32>, vec2<f32>, vec2<i32>) -> vec4<f32>
textureSampleGrad(texture_3d<f32>, sampler, vec3<f32>, vec3<f32>, vec3<f32>) -> vec4<f32>
textureSampleGrad(texture_3d<f32>, sampler, vec3<f32>, vec3<f32>, vec3<f32>, vec3<i32>) -> vec4<f32>
textureSampleGrad(texture_cube<f32>, sampler, vec3<f32>, vec3<f32>, vec3<f32>) -> vec4<f32>
textureSampleGrad(texture_cube_array<f32>, sampler, vec3<f32>, i32, vec3<f32>, vec3<f32>) -> vec4<f32>

textureSampleLevel(texture_2d<f32>, sampler, vec2<f32>, f32) -> vec4<f32>
textureSampleLevel(texture_2d<f32>, sampler, vec2<f32>, f32, vec2<i32>) -> vec4<f32>
textureSampleLevel(texture_2d_array<f32>, sampler, vec2<f32>, i32, f32) -> vec4<f32>
textureSampleLevel(texture_2d_array<f32>, sampler, vec2<f32>, i32, f32, vec2<i32>) -> vec4<f32>
textureSampleLevel(texture_3d<f32>, sampler, vec3<f32>, f32) -> vec4<f32>
textureSampleLevel(texture_3d<f32>, sampler, vec3<f32>, f32, vec3<i32>) -> vec4<f32>
textureSampleLevel(texture_cube<f32>, sampler, vec3<f32>, f32) -> vec4<f32>
textureSampleLevel(texture_cube_array<f32>, sampler, vec3<f32>, i32, f32) -> vec4<f32>
textureSampleLevel(texture_depth_2d, sampler, vec2<f32>, i32) -> f32
textureSampleLevel(texture_depth_2d, sampler, vec2<f32>, i32, vec2<i32>) -> f32
textureSampleLevel(texture_depth_2d_array, sampler, vec2<f32>, i32, i32) -> f32
textureSampleLevel(texture_depth_2d_array, sampler, vec2<f32>, i32, i32, vec2<i32>) -> f32
textureSampleLevel(texture_depth_cube, sampler, vec3<f32>, i32) -> f32
textureSampleLevel(texture_depth_cube_array, sampler, vec3<f32>, i32, i32) -> f32
textureSampleLevel(texture_external, sampler, vec2<f32>) -> vec4<f32>

textureStore(texture_storage_1d<F;write>, i32, F::StorageType)
textureStore(texture_storage_2d<F;write>, vec2<i32>, F::StorageType)
textureStore(texture_storage_2d_array<F;write>, vec2<i32>, i32, F::StorageType)
textureStore(texture_storage_3d<F;write>, vec3<i32>, F::StorageType)

// TODO atomic
atomicLoad(ptr<atomic<T>>) -> T
atomicStore(ptr<atomic<T>>, T)

atomicAdd(ptr<atomic<T>>, T) -> T
atomicSub(ptr<atomic<T>>, T) -> T
atomicMax(ptr<atomic<T>>, T) -> T
atomicMin(ptr<atomic<T>>, T) -> T
atomicAnd(ptr<atomic<T>>, T) -> T
atomicOr(ptr<atomic<T>>, T) -> T
atomicXor(ptr<atomic<T>>, T) -> T
atomicExchange(ptr<atomic<T>>, T) -> T
// TODO atomicCompareExchangeWeak

// data packing
pack4x8snorm(vec4<f32>) -> u32
pack4x8unorm(vec4<f32>) -> u32
pack2x16snorm(vec2<f32>) -> u32
pack2x16unorm(vec2<f32>) -> u32
pack2x16float(vec2<f32>) -> u32
unpack4x8snorm(u32) -> vec4<f32>
unpack4x8unorm(u32) -> vec4<f32>
unpack2x16snorm(u32) -> vec2<f32>
unpack2x16unorm(u32) -> vec2<f32>
unpack2x16float(u32) -> vec2<f32>

// synchronization
storageBarrier()
workgroupBarrier()


// TODO: T should only match i32, f32, u32 in a lot of ops

op_unary_minus(f32) -> f32
op_unary_minus(i32) -> i32
op_unary_minus(vecN<f32>) -> vecN<f32>
op_unary_minus(vecN<i32>) -> vecN<i32>

op_unary_not(bool) -> bool
op_unary_not(vecN<bool>) -> vecN<bool>

op_unary_bitnot(vecN<T>) -> vecN<T>
op_unary_bitnot(T) -> T

op_binary_bool(bool, bool) -> bool

op_eq(vecN<T>, vecN<T>) -> vecN<bool>
op_eq(T, T) -> bool

op_cmp(vecN<T>, vecN<T>) -> vecN<bool>
op_cmp(T, T) -> bool


op_binary_bitop(vecN<u32>, vecN<u32>) -> vecN<u32>
op_binary_bitop(u32, u32) -> u32
op_binary_bitop(vecN<i32>, vecN<i32>) -> vecN<i32>
op_binary_bitop(i32, i32) -> i32

op_binary_shift(vecN<u32>, vecN<u32>) -> vecN<u32>
op_binary_shift(u32, u32) -> u32
op_binary_shift(vecN<i32>, vecN<u32>) -> vecN<u32>
op_binary_shift(i32, u32) -> i32


op_binary_number(vecN<T>, vecN<T>) -> vecN<T>
op_binary_number(vecN<T>, T) -> vecN<T>
op_binary_number(T, vecN<T>) -> vecN<T>
op_binary_number(matMxN<f32>, matMxN<f32>) -> matMxN<f32>
op_binary_number(T, T) -> T

op_binary_div(vecN<T>, vecN<T>) -> vecN<T>
op_binary_div(vecN<T>, T) -> vecN<T>
op_binary_div(T, vecN<T>) -> vecN<T>
op_binary_div(T, T) -> T

op_binary_mul(matMxN<f32>, matMxN<f32>) -> matMxN<f32>
op_binary_mul(matMxN<f32>, vecM<f32>) -> vecN<f32>
op_binary_mul(vecN<f32>, matMxN<f32>) -> vecM<f32>
op_binary_mul(matMxN<f32>, f32) -> matMxN<f32>
op_binary_mul(f32, matMxN<f32>) -> matMxN<f32>
op_binary_mul(matKxN<f32>, matMxK<f32>) -> matMxN<f32>
op_binary_mul(vecN<T>, vecN<T>) -> vecN<T>
op_binary_mul(vecN<T>, vecN<T>) -> vecN<T>
op_binary_mul(vecN<T>, T) -> vecN<T>
op_binary_mul(T, vecN<T>) -> vecN<T>
op_binary_mul(T, T) -> T
