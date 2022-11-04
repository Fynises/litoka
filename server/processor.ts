const getRawClip = (url: string) => {
  return url.split('-preview-')[0] + '.mp4';
};

export {
  getRawClip
};