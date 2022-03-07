-- Your SQL goes here
CREATE TABLE `dwz` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `short_url` varchar(128) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '短地址',
  `redirect_url` varchar(2048) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '跳转地址',
  `valid_time` datetime NOT NULL DEFAULT '9999-12-31 23:59:59' COMMENT '有效期',
  `create_time` datetime NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uniq_short_url` (`short_url`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
